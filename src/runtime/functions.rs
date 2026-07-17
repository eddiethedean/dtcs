//! Stdlib function execution.

use crate::runtime::format_gate::validate_dtcs_format;
use crate::runtime::model::RuntimeValue;
use chrono::{DateTime, LocalResult, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use chrono_tz::Tz;
use indexmap::IndexMap;
use std::sync::OnceLock;

/// Evaluate a `dtcs:` function call.
pub fn call_function(callee: &str, args: &[RuntimeValue]) -> Result<RuntimeValue, String> {
    match callee {
        "dtcs:lower" => {
            let value = args.first().ok_or("dtcs:lower requires one argument")?;
            match value {
                RuntimeValue::Null => Ok(RuntimeValue::Null),
                RuntimeValue::String(s) => Ok(RuntimeValue::String(s.to_lowercase())),
                other => Err(format!("dtcs:lower requires string, got {other:?}")),
            }
        }
        "dtcs:upper" => {
            let value = args.first().ok_or("dtcs:upper requires one argument")?;
            match value {
                RuntimeValue::Null => Ok(RuntimeValue::Null),
                RuntimeValue::String(s) => Ok(RuntimeValue::String(s.to_uppercase())),
                other => Err(format!("dtcs:upper requires string, got {other:?}")),
            }
        }
        "dtcs:trim" | "dtcs:ltrim" | "dtcs:rtrim" => {
            let text = string_arg(args, 0, callee)?;
            let charset = args
                .get(1)
                .map(|value| string_value(value, callee))
                .transpose()?;
            let trim = |value: &str| match charset {
                Some(chars) => value.trim_matches(|c| chars.contains(c)).to_string(),
                None => value.trim().to_string(),
            };
            let out = match callee {
                "dtcs:ltrim" => match charset {
                    Some(chars) => text.trim_start_matches(|c| chars.contains(c)).to_string(),
                    None => text.trim_start().to_string(),
                },
                "dtcs:rtrim" => match charset {
                    Some(chars) => text.trim_end_matches(|c| chars.contains(c)).to_string(),
                    None => text.trim_end().to_string(),
                },
                _ => trim(text),
            };
            Ok(RuntimeValue::String(out))
        }
        "dtcs:normalize_whitespace" => Ok(RuntimeValue::String(
            string_arg(args, 0, callee)?
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" "),
        )),
        "dtcs:split" => {
            let text = string_arg(args, 0, callee)?;
            let separator = string_arg(args, 1, callee)?;
            Ok(RuntimeValue::List(
                text.split(separator)
                    .map(|part| RuntimeValue::String(part.into()))
                    .collect(),
            ))
        }
        "dtcs:join_strings" => {
            let RuntimeValue::List(values) =
                args.first().ok_or("dtcs:join_strings requires values")?
            else {
                return Err("dtcs:join_strings requires a list".into());
            };
            let separator = string_arg(args, 1, callee)?;
            let mut parts = Vec::with_capacity(values.len());
            for value in values {
                parts.push(string_value(value, callee)?.to_string());
            }
            Ok(RuntimeValue::String(parts.join(separator)))
        }
        "dtcs:pad_left" | "dtcs:pad_right" => {
            let text = string_arg(args, 0, callee)?;
            let target = args
                .get(1)
                .and_then(RuntimeValue::as_integer)
                .ok_or("pad length must be integer")?;
            if target < 0 {
                return Err("pad length must be non-negative".into());
            }
            let fill = args
                .get(2)
                .map(|value| string_value(value, callee))
                .transpose()?
                .unwrap_or(" ");
            if fill.is_empty() {
                return Err("pad fill must not be empty".into());
            }
            let count = text.chars().count();
            let padding = fill
                .chars()
                .cycle()
                .take((target as usize).saturating_sub(count))
                .collect::<String>();
            Ok(RuntimeValue::String(if callee == "dtcs:pad_left" {
                format!("{padding}{text}")
            } else {
                format!("{text}{padding}")
            }))
        }
        "dtcs:repeat" => {
            let text = string_arg(args, 0, callee)?;
            let count = args
                .get(1)
                .and_then(RuntimeValue::as_integer)
                .ok_or("repeat count must be integer")?;
            if !(0..=1_000_000).contains(&count) {
                return Err("repeat count exceeds budget".into());
            }
            Ok(RuntimeValue::String(text.repeat(count as usize)))
        }
        "dtcs:reverse" => Ok(RuntimeValue::String(
            string_arg(args, 0, callee)?.chars().rev().collect(),
        )),
        "dtcs:position" => {
            let text = string_arg(args, 0, callee)?;
            let needle = string_arg(args, 1, callee)?;
            Ok(RuntimeValue::Integer(
                text.find(needle)
                    .map(|byte| text[..byte].chars().count() as i64)
                    .unwrap_or(-1),
            ))
        }
        "dtcs:lower_unicode" | "dtcs:casefold" => Ok(RuntimeValue::String(
            string_arg(args, 0, callee)?.to_lowercase(),
        )),
        "dtcs:upper_unicode" => Ok(RuntimeValue::String(
            string_arg(args, 0, callee)?.to_uppercase(),
        )),
        "dtcs:regex_matches"
        | "dtcs:regex_contains"
        | "dtcs:regex_replace"
        | "dtcs:regex_extract"
        | "dtcs:regex_extract_all"
        | "dtcs:regex_split" => {
            let text = string_arg(args, 0, callee)?;
            if text.len() > 1_048_576 {
                return Err("regex input exceeds budget".into());
            }
            let pattern = string_arg(args, 1, callee)?;
            let regex = super::regex_gate::compile_dtcs_regex(pattern)?;
            match callee {
                "dtcs:regex_matches" => {
                    Ok(RuntimeValue::Boolean(regex.find(text).is_some_and(
                        |found| found.start() == 0 && found.end() == text.len(),
                    )))
                }
                "dtcs:regex_contains" => Ok(RuntimeValue::Boolean(regex.is_match(text))),
                "dtcs:regex_replace" => Ok(RuntimeValue::String(
                    regex
                        .replace_all(text, string_arg(args, 2, callee)?)
                        .into_owned(),
                )),
                "dtcs:regex_extract" => {
                    let group = args.get(2).and_then(RuntimeValue::as_integer).unwrap_or(0);
                    Ok(match regex.captures(text) {
                        Some(caps) => caps
                            .get(group as usize)
                            .map(|m| RuntimeValue::String(m.as_str().into()))
                            .unwrap_or(RuntimeValue::Null),
                        None => RuntimeValue::Null,
                    })
                }
                "dtcs:regex_extract_all" => {
                    let group =
                        args.get(2).and_then(RuntimeValue::as_integer).unwrap_or(0) as usize;
                    Ok(RuntimeValue::List(regex_extract_all_advancing(
                        &regex, text, group,
                    )))
                }
                "dtcs:regex_split" => Ok(RuntimeValue::List(
                    regex_split_advancing(&regex, text)
                        .into_iter()
                        .map(RuntimeValue::String)
                        .collect(),
                )),
                _ => unreachable!(),
            }
        }
        "dtcs:translate" => {
            let text = string_arg(args, 0, callee)?;
            let from = string_arg(args, 1, callee)?;
            let to = string_arg(args, 2, callee)?;
            let from_chars: Vec<char> = from.chars().collect();
            let to_chars: Vec<char> = to.chars().collect();
            Ok(RuntimeValue::String(
                text.chars()
                    .map(|c| {
                        from_chars
                            .iter()
                            .position(|f| *f == c)
                            .and_then(|i| to_chars.get(i).copied())
                            .unwrap_or(c)
                    })
                    .collect(),
            ))
        }
        "dtcs:parse_date" => {
            let text = string_arg(args, 0, callee)?;
            let format = args
                .get(1)
                .map(|v| string_value(v, callee))
                .transpose()?
                .unwrap_or("%Y-%m-%d");
            validate_dtcs_format(format)?;
            NaiveDate::parse_from_str(text, format)
                .map(|d| RuntimeValue::Date(d.format("%Y-%m-%d").to_string()))
                .map_err(|_| format!("{callee} failed to parse date"))
        }
        "dtcs:parse_time" => {
            let text = string_arg(args, 0, callee)?;
            let format = args
                .get(1)
                .map(|v| string_value(v, callee))
                .transpose()?
                .unwrap_or("%H:%M:%S");
            validate_dtcs_format(format)?;
            NaiveTime::parse_from_str(text, format)
                .map(|t| RuntimeValue::Time(t.format("%H:%M:%S").to_string()))
                .map_err(|_| format!("{callee} failed to parse time"))
        }
        "dtcs:parse_datetime" => {
            let text = string_arg(args, 0, callee)?;
            if let Ok(dt) = DateTime::parse_from_rfc3339(text) {
                return Ok(RuntimeValue::DateTime(dt.to_rfc3339()));
            }
            let format = args
                .get(1)
                .map(|v| string_value(v, callee))
                .transpose()?
                .unwrap_or("%Y-%m-%dT%H:%M:%S");
            validate_dtcs_format(format)?;
            NaiveDateTime::parse_from_str(text, format)
                .map(|d| RuntimeValue::DateTime(d.format("%Y-%m-%dT%H:%M:%S").to_string()))
                .map_err(|_| format!("{callee} failed to parse datetime"))
        }
        "dtcs:format_date" => {
            let text = string_arg(args, 0, callee)?;
            let format = args
                .get(1)
                .map(|v| string_value(v, callee))
                .transpose()?
                .unwrap_or("%Y-%m-%d");
            validate_dtcs_format(format)?;
            NaiveDate::parse_from_str(text, "%Y-%m-%d")
                .map(|d| RuntimeValue::String(d.format(format).to_string()))
                .map_err(|_| format!("{callee} requires YYYY-MM-DD date"))
        }
        "dtcs:format_datetime" => {
            let text = string_arg(args, 0, callee)?;
            let format = args
                .get(1)
                .map(|v| string_value(v, callee))
                .transpose()?
                .unwrap_or("%Y-%m-%dT%H:%M:%S");
            validate_dtcs_format(format)?;
            if let Ok(dt) = DateTime::parse_from_rfc3339(text) {
                return Ok(RuntimeValue::String(dt.format(format).to_string()));
            }
            NaiveDateTime::parse_from_str(text, "%Y-%m-%dT%H:%M:%S")
                .map(|d| RuntimeValue::String(d.format(format).to_string()))
                .map_err(|_| format!("{callee} requires datetime value"))
        }
        "dtcs:list_position" => {
            let RuntimeValue::List(items) =
                args.first().ok_or("dtcs:list_position requires list")?
            else {
                return Err("dtcs:list_position requires list".into());
            };
            let needle = args.get(1).ok_or("dtcs:list_position requires value")?;
            Ok(RuntimeValue::Integer(
                items
                    .iter()
                    .position(|item| item == needle)
                    .map(|i| i as i64)
                    .unwrap_or(-1),
            ))
        }
        "dtcs:list_slice" => {
            let RuntimeValue::List(items) = args.first().ok_or("dtcs:list_slice requires list")?
            else {
                return Err("dtcs:list_slice requires list".into());
            };
            let start = args
                .get(1)
                .and_then(RuntimeValue::as_integer)
                .unwrap_or(0)
                .max(0) as usize;
            let end = args
                .get(2)
                .and_then(RuntimeValue::as_integer)
                .map(|v| v.max(0) as usize)
                .unwrap_or(items.len())
                .min(items.len());
            let start = start.min(end);
            Ok(RuntimeValue::List(items[start..end].to_vec()))
        }
        "dtcs:list_sort" => {
            let RuntimeValue::List(mut items) = args
                .first()
                .cloned()
                .ok_or("dtcs:list_sort requires list")?
            else {
                return Err("dtcs:list_sort requires list".into());
            };
            items.sort_by(|a, b| compare_ordered_values(a, b).unwrap_or(std::cmp::Ordering::Equal));
            Ok(RuntimeValue::List(items))
        }
        "dtcs:map_entries" => match args.first().ok_or("dtcs:map_entries requires map")? {
            RuntimeValue::Map(map) => Ok(RuntimeValue::List(
                map.iter()
                    .map(|(k, v)| {
                        RuntimeValue::List(vec![RuntimeValue::String(k.clone()), v.clone()])
                    })
                    .collect(),
            )),
            RuntimeValue::Null | RuntimeValue::Missing(_) => Ok(RuntimeValue::Null),
            _ => Err("dtcs:map_entries requires map".into()),
        },
        "dtcs:map_from_entries" => {
            let RuntimeValue::List(entries) =
                args.first().ok_or("dtcs:map_from_entries requires list")?
            else {
                return Err("dtcs:map_from_entries requires list".into());
            };
            let mut map = IndexMap::new();
            for entry in entries {
                let RuntimeValue::List(pair) = entry else {
                    return Err("dtcs:map_from_entries requires [key, value] pairs".into());
                };
                if pair.len() != 2 {
                    return Err("dtcs:map_from_entries requires [key, value] pairs".into());
                }
                let key = string_value(&pair[0], callee)?.to_string();
                if map.contains_key(&key) {
                    return Err(format!("dtcs:map_from_entries duplicate key '{key}'"));
                }
                map.insert(key, pair[1].clone());
            }
            Ok(RuntimeValue::Map(map))
        }
        "dtcs:map_concat" => {
            let (maps, policy) = split_map_concat_args(args)?;
            let mut out = IndexMap::new();
            for map in maps {
                for (key, value) in map {
                    match policy {
                        None | Some("right") => {
                            out.insert(key, value);
                        }
                        Some("left") => {
                            out.entry(key).or_insert(value);
                        }
                        Some("error") => {
                            if out.contains_key(&key) {
                                return Err(format!(
                                    "dtcs:map_concat duplicate key '{key}' under collisionPolicy error"
                                ));
                            }
                            out.insert(key, value);
                        }
                        Some(other) => {
                            return Err(format!(
                                "dtcs:map_concat unsupported collisionPolicy '{other}'"
                            ));
                        }
                    }
                }
            }
            Ok(RuntimeValue::Map(out))
        }
        "dtcs:object_names" => match args.first().ok_or("dtcs:object_names requires object")? {
            RuntimeValue::Map(map) => Ok(RuntimeValue::List(
                map.keys().cloned().map(RuntimeValue::String).collect(),
            )),
            RuntimeValue::Null | RuntimeValue::Missing(_) => Ok(RuntimeValue::Null),
            _ => Err("dtcs:object_names requires object".into()),
        },
        "dtcs:object_values" => match args.first().ok_or("dtcs:object_values requires object")? {
            RuntimeValue::Map(map) => Ok(RuntimeValue::List(map.values().cloned().collect())),
            RuntimeValue::Null | RuntimeValue::Missing(_) => Ok(RuntimeValue::Null),
            _ => Err("dtcs:object_values requires object".into()),
        },
        "dtcs:variance" | "dtcs:stddev" => {
            let (nums, population) = stats_numbers_and_mode(args)?;
            if nums.len() < 2 {
                return Ok(RuntimeValue::Null);
            }
            let n = nums.len() as f64;
            let mean = nums.iter().sum::<f64>() / n;
            let denom = if population { n } else { n - 1.0 };
            let var = nums.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / denom;
            Ok(if callee == "dtcs:stddev" {
                RuntimeValue::Decimal(var.sqrt())
            } else {
                RuntimeValue::Decimal(var)
            })
        }
        "dtcs:covariance" => {
            let (xs, ys, population) = covariance_args(args)?;
            let pairs: Vec<(f64, f64)> = xs
                .iter()
                .zip(ys.iter())
                .filter_map(|(x, y)| Some((x.as_decimal()?, y.as_decimal()?)))
                .collect();
            if pairs.len() < 2 {
                return Ok(RuntimeValue::Null);
            }
            let n = pairs.len() as f64;
            let mx = pairs.iter().map(|(x, _)| x).sum::<f64>() / n;
            let my = pairs.iter().map(|(_, y)| y).sum::<f64>() / n;
            let denom = if population { n } else { n - 1.0 };
            let cov = pairs.iter().map(|(x, y)| (x - mx) * (y - my)).sum::<f64>() / denom;
            Ok(RuntimeValue::Decimal(cov))
        }
        "dtcs:correlation" => {
            if args.len() != 2 {
                return Err("dtcs:correlation requires two lists".into());
            }
            let (RuntimeValue::List(xs), RuntimeValue::List(ys)) = (&args[0], &args[1]) else {
                return Err("dtcs:correlation requires two lists".into());
            };
            let pairs: Vec<(f64, f64)> = xs
                .iter()
                .zip(ys.iter())
                .filter_map(|(x, y)| Some((x.as_decimal()?, y.as_decimal()?)))
                .collect();
            if pairs.len() < 2 {
                return Ok(RuntimeValue::Null);
            }
            let n = pairs.len() as f64;
            let mx = pairs.iter().map(|(x, _)| x).sum::<f64>() / n;
            let my = pairs.iter().map(|(_, y)| y).sum::<f64>() / n;
            let mut num = 0.0;
            let mut dx = 0.0;
            let mut dy = 0.0;
            for (x, y) in &pairs {
                let a = x - mx;
                let b = y - my;
                num += a * b;
                dx += a * a;
                dy += b * b;
            }
            if dx == 0.0 || dy == 0.0 {
                return Ok(RuntimeValue::Null);
            }
            Ok(RuntimeValue::Decimal(num / (dx.sqrt() * dy.sqrt())))
        }
        "dtcs:median" => {
            let mut nums: Vec<f64> = args.iter().filter_map(RuntimeValue::as_decimal).collect();
            if nums.is_empty() {
                return Ok(RuntimeValue::Null);
            }
            nums.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            let mid = nums.len() / 2;
            Ok(RuntimeValue::Decimal(if nums.len() % 2 == 0 {
                (nums[mid - 1] + nums[mid]) / 2.0
            } else {
                nums[mid]
            }))
        }
        "dtcs:quantile" => {
            let q = args
                .last()
                .and_then(RuntimeValue::as_decimal)
                .ok_or("dtcs:quantile requires quantile in [0,1]")?;
            if !(0.0..=1.0).contains(&q) {
                return Err("dtcs:quantile requires quantile in [0,1]".into());
            }
            let mut nums: Vec<f64> = args[..args.len().saturating_sub(1)]
                .iter()
                .filter_map(RuntimeValue::as_decimal)
                .collect();
            if nums.is_empty() {
                return Ok(RuntimeValue::Null);
            }
            nums.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            // nearest-rank
            let rank = ((q * nums.len() as f64).ceil() as usize).max(1);
            Ok(RuntimeValue::Decimal(nums[rank - 1]))
        }
        "dtcs:first" => Ok(args.first().cloned().unwrap_or(RuntimeValue::Null)),
        "dtcs:last" => Ok(args.last().cloned().unwrap_or(RuntimeValue::Null)),
        "dtcs:collect_list" => {
            if args.len() > 1_000_000 {
                return Err("collect_list exceeds collectionElements budget".into());
            }
            Ok(RuntimeValue::List(args.to_vec()))
        }
        "dtcs:collect_set" => {
            let mut out = Vec::new();
            for value in args {
                if !out.iter().any(|existing| existing == value) {
                    out.push(value.clone());
                }
                if out.len() > 1_000_000 {
                    return Err("collect_set exceeds collectionElements budget".into());
                }
            }
            Ok(RuntimeValue::List(out))
        }
        "dtcs:cast" | "dtcs:try_cast" => {
            if callee == "dtcs:cast" && args.len() != 2 {
                return Err(format!("{callee} requires value and target type"));
            }
            if callee == "dtcs:try_cast" && !(2..=3).contains(&args.len()) {
                return Err(format!(
                    "{callee} requires value, target type, and optional policy"
                ));
            }
            let target = string_arg(args, 1, callee)?;
            let policy = if callee == "dtcs:try_cast" {
                args.get(2)
                    .map(|v| string_value(v, callee))
                    .transpose()?
                    .unwrap_or("invalid")
            } else {
                "fail"
            };
            let result = cast_runtime_value(&args[0], target);
            match (callee, result) {
                (_, Ok(value)) => Ok(value),
                ("dtcs:try_cast", Err(error)) => match policy {
                    "invalid" => Ok(RuntimeValue::invalid("cast failed")),
                    "null" => Ok(RuntimeValue::Null),
                    "fail" => Err(error),
                    other => Err(format!("unsupported try_cast policy '{other}'")),
                },
                (_, Err(error)) => Err(error),
            }
        }
        "dtcs:parse_decimal" => string_arg(args, 0, callee)?
            .parse::<f64>()
            .map(RuntimeValue::Decimal)
            .map_err(|_| "dtcs:parse_decimal failed".into()),
        "dtcs:parse_boolean" => match string_arg(args, 0, callee)?.to_ascii_lowercase().as_str() {
            "true" | "1" | "yes" => Ok(RuntimeValue::Boolean(true)),
            "false" | "0" | "no" => Ok(RuntimeValue::Boolean(false)),
            _ => Err("dtcs:parse_boolean failed".into()),
        },
        "dtcs:list" | "dtcs:tuple" => Ok(RuntimeValue::List(args.to_vec())),
        "dtcs:map" | "dtcs:object" => {
            if args.len() % 2 != 0 {
                return Err(format!("{callee} requires key/value pairs"));
            }
            let mut map = IndexMap::new();
            for pair in args.chunks(2) {
                let key = string_value(&pair[0], callee)?;
                if map.insert(key.into(), pair[1].clone()).is_some() {
                    return Err(format!("{callee} rejects duplicate keys"));
                }
            }
            Ok(RuntimeValue::Map(map))
        }
        "dtcs:size" => match args.first().ok_or("dtcs:size requires value")? {
            RuntimeValue::List(items) => Ok(RuntimeValue::Integer(items.len() as i64)),
            RuntimeValue::Map(items) => Ok(RuntimeValue::Integer(items.len() as i64)),
            RuntimeValue::String(text) => Ok(RuntimeValue::Integer(text.chars().count() as i64)),
            RuntimeValue::Null | RuntimeValue::Missing(_) => Ok(RuntimeValue::Null),
            other => Err(format!("dtcs:size unsupported type {other:?}")),
        },
        "dtcs:list_contains" => {
            let RuntimeValue::List(items) =
                args.first().ok_or("dtcs:list_contains requires list")?
            else {
                return Err("dtcs:list_contains requires list".into());
            };
            let value = args.get(1).ok_or("dtcs:list_contains requires value")?;
            Ok(RuntimeValue::Boolean(
                items.iter().any(|item| item == value),
            ))
        }
        "dtcs:list_concat" => {
            let mut out = Vec::new();
            for value in args {
                let RuntimeValue::List(items) = value else {
                    return Err("dtcs:list_concat requires lists".into());
                };
                out.extend(items.clone());
            }
            Ok(RuntimeValue::List(out))
        }
        "dtcs:map_keys" => match args.first().ok_or("dtcs:map_keys requires map")? {
            RuntimeValue::Map(map) => Ok(RuntimeValue::List(
                map.keys().cloned().map(RuntimeValue::String).collect(),
            )),
            RuntimeValue::Null | RuntimeValue::Missing(_) => Ok(RuntimeValue::Null),
            _ => Err("dtcs:map_keys requires map".into()),
        },
        "dtcs:map_values" => match args.first().ok_or("dtcs:map_values requires map")? {
            RuntimeValue::Map(map) => Ok(RuntimeValue::List(map.values().cloned().collect())),
            RuntimeValue::Null | RuntimeValue::Missing(_) => Ok(RuntimeValue::Null),
            _ => Err("dtcs:map_values requires map".into()),
        },
        "dtcs:to_timezone" | "dtcs:from_utc" => {
            let timestamp = string_arg(args, 0, callee)?;
            let zone = parse_timezone(string_arg(args, 1, callee)?)?;
            let instant = DateTime::parse_from_rfc3339(timestamp)
                .map_err(|_| format!("{callee} requires an RFC 3339 timestamp"))?;
            Ok(RuntimeValue::DateTime(
                instant.with_timezone(&zone).to_rfc3339(),
            ))
        }
        "dtcs:to_utc" => {
            let local = string_arg(args, 0, callee)?;
            let zone = parse_timezone(string_arg(args, 1, callee)?)?;
            let ambiguous = args
                .get(2)
                .map(|v| string_value(v, callee))
                .transpose()?
                .unwrap_or("reject");
            let nonexistent = args
                .get(3)
                .map(|v| string_value(v, callee))
                .transpose()?
                .unwrap_or("reject");
            let naive = NaiveDateTime::parse_from_str(local, "%Y-%m-%dT%H:%M:%S")
                .or_else(|_| NaiveDateTime::parse_from_str(local, "%Y-%m-%dT%H:%M:%S%.f"))
                .map_err(|_| "dtcs:to_utc requires local YYYY-MM-DDTHH:MM:SS value".to_string())?;
            let zoned = match zone.from_local_datetime(&naive) {
                LocalResult::Single(value) => value,
                LocalResult::Ambiguous(earliest, latest) => match ambiguous {
                    "earliest" | "earlier" => earliest,
                    "latest" | "later" => latest,
                    "reject" | "error" => {
                        return Err(
                            "dtcs:to_utc rejects ambiguous local time without explicit policy"
                                .into(),
                        )
                    }
                    other => return Err(format!("unsupported ambiguousPolicy '{other}'")),
                },
                LocalResult::None => match nonexistent {
                    "shift_forward" | "shiftForward" => zone
                        .from_local_datetime(&naive)
                        .latest()
                        .or_else(|| {
                            // Fall forward by one hour when the local instant does not exist.
                            zone.from_local_datetime(&(naive + chrono::Duration::hours(1)))
                                .single()
                        })
                        .ok_or_else(|| {
                            "dtcs:to_utc could not shift nonexistent local time forward".to_string()
                        })?,
                    "reject" | "error" => {
                        return Err(
                            "dtcs:to_utc rejects nonexistent local time without explicit policy"
                                .into(),
                        )
                    }
                    other => return Err(format!("unsupported nonexistentPolicy '{other}'")),
                },
            };
            Ok(RuntimeValue::DateTime(
                zoned.with_timezone(&Utc).to_rfc3339(),
            ))
        }
        "dtcs:random" => {
            let seed = args
                .first()
                .and_then(RuntimeValue::as_integer)
                .map(|value| value as u64)
                .unwrap_or_else(|| uuid::Uuid::new_v4().as_u128() as u64);
            Ok(RuntimeValue::Decimal(unit_random(seed)))
        }
        "dtcs:random_normal" => {
            let seed = args
                .first()
                .and_then(RuntimeValue::as_integer)
                .map(|value| value as u64)
                .unwrap_or_else(|| uuid::Uuid::new_v4().as_u128() as u64);
            let mean = args
                .get(1)
                .and_then(RuntimeValue::as_decimal)
                .unwrap_or(0.0);
            let deviation = args
                .get(2)
                .and_then(RuntimeValue::as_decimal)
                .unwrap_or(1.0);
            if !(deviation.is_finite() && deviation >= 0.0) {
                return Err(
                    "dtcs:random_normal requires non-negative finite standard deviation".into(),
                );
            }
            let u1 = unit_random(seed).max(f64::MIN_POSITIVE);
            let u2 = unit_random(seed ^ 0x9E37_79B9_7F4A_7C15);
            Ok(RuntimeValue::Decimal(
                mean + deviation
                    * (-2.0 * u1.ln()).sqrt()
                    * (2.0 * std::f64::consts::PI * u2).cos(),
            ))
        }
        "dtcs:uuid" => Ok(RuntimeValue::String(
            uuid::Uuid::new_v4().to_string().to_ascii_lowercase(),
        )),
        "dtcs:run_id" => Ok(RuntimeValue::String(run_id().clone())),
        "dtcs:run_timestamp" => Ok(RuntimeValue::DateTime(run_timestamp().clone())),
        "dtcs:concat" => {
            if args.len() < 2 {
                return Err("dtcs:concat requires at least two arguments".into());
            }
            let mut out = String::new();
            for arg in args {
                let RuntimeValue::String(s) = arg else {
                    return Err("dtcs:concat requires string arguments".into());
                };
                out.push_str(s);
            }
            Ok(RuntimeValue::String(out))
        }
        "dtcs:substr" => {
            let s = args
                .first()
                .ok_or("dtcs:substr requires at least two arguments")?;
            let start = args
                .get(1)
                .and_then(RuntimeValue::as_integer)
                .ok_or("dtcs:substr start must be integer")?;
            if start < 0 {
                return Err("dtcs:substr start must be non-negative".into());
            }
            match s {
                RuntimeValue::Null => Ok(RuntimeValue::Null),
                RuntimeValue::String(text) => {
                    let chars: Vec<char> = text.chars().collect();
                    let start = start as usize;
                    if start >= chars.len() {
                        return Ok(RuntimeValue::String(String::new()));
                    }
                    let slice = if let Some(len_arg) = args.get(2) {
                        let len = len_arg
                            .as_integer()
                            .ok_or("dtcs:substr length must be integer")?;
                        if len < 0 {
                            return Err("dtcs:substr length must be non-negative".into());
                        }
                        let end = (start + len as usize).min(chars.len());
                        &chars[start..end]
                    } else {
                        &chars[start..]
                    };
                    Ok(RuntimeValue::String(slice.iter().collect()))
                }
                other => Err(format!("dtcs:substr requires string, got {other:?}")),
            }
        }
        "dtcs:replace" => {
            if args.len() < 3 {
                return Err("dtcs:replace requires three arguments".into());
            }
            let RuntimeValue::String(text) = &args[0] else {
                return Err("dtcs:replace requires string haystack".into());
            };
            let RuntimeValue::String(from) = &args[1] else {
                return Err("dtcs:replace requires string needle".into());
            };
            let RuntimeValue::String(to) = &args[2] else {
                return Err("dtcs:replace requires string replacement".into());
            };
            Ok(RuntimeValue::String(text.replace(from, to)))
        }
        "dtcs:coalesce" => {
            for arg in args {
                if !arg.is_null() && !arg.is_missing() && !arg.is_invalid() {
                    return Ok(arg.clone());
                }
            }
            Ok(RuntimeValue::Null)
        }
        "dtcs:between" | "between" => {
            if args.len() != 3 {
                return Err("dtcs:between requires value, lower, and upper".into());
            }
            eval_between(&args[0], &args[1], &args[2])
        }
        "dtcs:field" | "field" => {
            if args.len() != 2 {
                return Err("dtcs:field requires object/map and field name".into());
            }
            eval_field_access(&args[0], &args[1])
        }
        "dtcs:index" | "index" => {
            if args.len() != 2 {
                return Err("dtcs:index requires collection and index".into());
            }
            eval_index_access(&args[0], &args[1], false)
        }
        "dtcs:element_at" | "element_at" | "elementAt" => {
            if args.len() != 2 {
                return Err("dtcs:element_at requires collection and index/key".into());
            }
            eval_index_access(&args[0], &args[1], true)
        }
        "dtcs:length" => {
            let value = args.first().ok_or("dtcs:length requires one argument")?;
            let len = match value {
                RuntimeValue::Null => {
                    return Err("dtcs:length does not accept null".into());
                }
                RuntimeValue::String(s) => s.chars().count() as i64,
                RuntimeValue::Binary(b) => b.len() as i64,
                other => return Err(format!("dtcs:length unsupported type {other:?}")),
            };
            Ok(RuntimeValue::Integer(len))
        }
        "dtcs:to_string" => {
            let value = args.first().ok_or("dtcs:to_string requires one argument")?;
            Ok(RuntimeValue::String(match value {
                RuntimeValue::Null | RuntimeValue::Missing(_) => return Ok(RuntimeValue::Null),
                RuntimeValue::Invalid(_) => return Ok(RuntimeValue::Null),
                RuntimeValue::String(s)
                | RuntimeValue::Binary(s)
                | RuntimeValue::Date(s)
                | RuntimeValue::Time(s)
                | RuntimeValue::DateTime(s)
                | RuntimeValue::Duration(s) => s.clone(),
                RuntimeValue::Boolean(b) => b.to_string(),
                RuntimeValue::Integer(i) => i.to_string(),
                RuntimeValue::Decimal(d) => d.to_string(),
                RuntimeValue::List(_) | RuntimeValue::Map(_) => {
                    return Err("dtcs:to_string unsupported for collections".into())
                }
            }))
        }
        "dtcs:to_integer" => {
            let value = args
                .first()
                .ok_or("dtcs:to_integer requires one argument")?;
            match value {
                RuntimeValue::Null => Ok(RuntimeValue::Null),
                RuntimeValue::Integer(i) => Ok(RuntimeValue::Integer(*i)),
                RuntimeValue::Decimal(d) => {
                    if d.fract() != 0.0 {
                        return Err("dtcs:to_integer requires integer-valued decimal".into());
                    }
                    Ok(RuntimeValue::Integer(*d as i64))
                }
                RuntimeValue::String(s) => s
                    .parse::<i64>()
                    .map(RuntimeValue::Integer)
                    .map_err(|_| "dtcs:to_integer parse failed".to_string()),
                other => Err(format!("dtcs:to_integer unsupported type {other:?}")),
            }
        }
        "dtcs:to_decimal" => {
            let value = args
                .first()
                .ok_or("dtcs:to_decimal requires one argument")?;
            match value {
                RuntimeValue::Null => Ok(RuntimeValue::Null),
                RuntimeValue::Decimal(d) => Ok(RuntimeValue::Decimal(*d)),
                RuntimeValue::Integer(i) => Ok(RuntimeValue::Decimal(*i as f64)),
                RuntimeValue::String(s) => s
                    .parse::<f64>()
                    .map(RuntimeValue::Decimal)
                    .map_err(|_| "dtcs:to_decimal parse failed".to_string()),
                other => Err(format!("dtcs:to_decimal unsupported type {other:?}")),
            }
        }
        "dtcs:abs" => {
            let value = args.first().ok_or("dtcs:abs requires one argument")?;
            match value {
                RuntimeValue::Null => Ok(RuntimeValue::Null),
                RuntimeValue::Integer(i) => Ok(RuntimeValue::Integer(i.abs())),
                RuntimeValue::Decimal(d) => Ok(RuntimeValue::Decimal(d.abs())),
                other => Err(format!("dtcs:abs requires numeric, got {other:?}")),
            }
        }
        "dtcs:min" => {
            if args.len() < 2 {
                return Err("dtcs:min requires at least two arguments".into());
            }
            let mut best: Option<f64> = None;
            for arg in args {
                if arg.is_null() {
                    return Ok(RuntimeValue::Null);
                }
                let Some(v) = arg.as_decimal() else {
                    return Err(format!("dtcs:min requires numeric, got {arg:?}"));
                };
                best = Some(best.map_or(v, |b| b.min(v)));
            }
            Ok(RuntimeValue::Decimal(best.unwrap()))
        }
        "dtcs:max" => {
            if args.len() < 2 {
                return Err("dtcs:max requires at least two arguments".into());
            }
            let mut best: Option<f64> = None;
            for arg in args {
                if arg.is_null() {
                    return Ok(RuntimeValue::Null);
                }
                let Some(v) = arg.as_decimal() else {
                    return Err(format!("dtcs:max requires numeric, got {arg:?}"));
                };
                best = Some(best.map_or(v, |b| b.max(v)));
            }
            Ok(RuntimeValue::Decimal(best.unwrap()))
        }
        "dtcs:contains" => {
            let haystack = args.first().ok_or("dtcs:contains requires two arguments")?;
            let needle = args.get(1).ok_or("dtcs:contains requires two arguments")?;
            match (haystack, needle) {
                (RuntimeValue::Null, _) | (_, RuntimeValue::Null) => Ok(RuntimeValue::Null),
                (RuntimeValue::String(h), RuntimeValue::String(n)) => {
                    Ok(RuntimeValue::Boolean(h.contains(n.as_str())))
                }
                _ => Err("dtcs:contains requires string arguments".into()),
            }
        }
        "dtcs:is_null" => {
            let value = args.first().ok_or("dtcs:is_null requires one argument")?;
            Ok(RuntimeValue::Boolean(value.is_null()))
        }
        "dtcs:is_missing" => {
            let value = args
                .first()
                .ok_or("dtcs:is_missing requires one argument")?;
            Ok(RuntimeValue::Boolean(value.is_missing()))
        }
        "dtcs:is_invalid" => {
            let value = args
                .first()
                .ok_or("dtcs:is_invalid requires one argument")?;
            Ok(RuntimeValue::Boolean(value.is_invalid()))
        }
        "dtcs:if_null" => {
            if args.len() != 2 {
                return Err("dtcs:if_null requires two arguments".into());
            }
            if args[0].is_null() || args[0].is_missing() {
                Ok(args[1].clone())
            } else {
                Ok(args[0].clone())
            }
        }
        "dtcs:null_if" => {
            if args.len() != 2 {
                return Err("dtcs:null_if requires two arguments".into());
            }
            if args[0] == args[1] {
                Ok(RuntimeValue::Null)
            } else {
                Ok(args[0].clone())
            }
        }
        "dtcs:case_when" => {
            if args.len() < 2 {
                return Err("dtcs:case_when requires at least two arguments".into());
            }
            let mut i = 0;
            while i + 1 < args.len() {
                match &args[i] {
                    RuntimeValue::Boolean(true) => return Ok(args[i + 1].clone()),
                    RuntimeValue::Boolean(false)
                    | RuntimeValue::Null
                    | RuntimeValue::Missing(_) => i += 2,
                    other if i + 1 == args.len() - 1 && i % 2 == 0 => {
                        // trailing else
                        return Ok(other.clone());
                    }
                    _ => i += 2,
                }
            }
            if args.len() % 2 == 1 {
                Ok(args[args.len() - 1].clone())
            } else {
                Ok(RuntimeValue::Null)
            }
        }
        "dtcs:concat_ws" => {
            if args.len() < 2 {
                return Err("dtcs:concat_ws requires separator and at least one value".into());
            }
            let sep = match &args[0] {
                RuntimeValue::String(s) => s.clone(),
                _ => return Err("dtcs:concat_ws separator must be string".into()),
            };
            let mut parts = Vec::new();
            for arg in &args[1..] {
                if let RuntimeValue::String(s) = arg {
                    parts.push(s.clone());
                }
            }
            Ok(RuntimeValue::String(parts.join(&sep)))
        }
        "dtcs:starts_with" => {
            let haystack = args
                .first()
                .ok_or("dtcs:starts_with requires two arguments")?;
            let needle = args
                .get(1)
                .ok_or("dtcs:starts_with requires two arguments")?;
            match (haystack, needle) {
                (RuntimeValue::Null, _) | (_, RuntimeValue::Null) => Ok(RuntimeValue::Null),
                (RuntimeValue::String(h), RuntimeValue::String(n)) => {
                    Ok(RuntimeValue::Boolean(h.starts_with(n.as_str())))
                }
                _ => Err("dtcs:starts_with requires string arguments".into()),
            }
        }
        "dtcs:ends_with" => {
            let haystack = args
                .first()
                .ok_or("dtcs:ends_with requires two arguments")?;
            let needle = args.get(1).ok_or("dtcs:ends_with requires two arguments")?;
            match (haystack, needle) {
                (RuntimeValue::Null, _) | (_, RuntimeValue::Null) => Ok(RuntimeValue::Null),
                (RuntimeValue::String(h), RuntimeValue::String(n)) => {
                    Ok(RuntimeValue::Boolean(h.ends_with(n.as_str())))
                }
                _ => Err("dtcs:ends_with requires string arguments".into()),
            }
        }
        "dtcs:round" => {
            let value = args.first().ok_or("dtcs:round requires one argument")?;
            let digits = args.get(1).and_then(RuntimeValue::as_integer).unwrap_or(0);
            match value {
                RuntimeValue::Null => Ok(RuntimeValue::Null),
                other => {
                    let v = other
                        .as_decimal()
                        .ok_or_else(|| format!("dtcs:round unsupported type {other:?}"))?;
                    let factor = 10f64.powi(digits as i32);
                    Ok(RuntimeValue::Decimal((v * factor).round() / factor))
                }
            }
        }
        "dtcs:floor" => {
            let value = args.first().ok_or("dtcs:floor requires one argument")?;
            match value.as_decimal() {
                Some(v) => Ok(RuntimeValue::Decimal(v.floor())),
                None if value.is_null() => Ok(RuntimeValue::Null),
                None => Err("dtcs:floor requires numeric".into()),
            }
        }
        "dtcs:ceil" => {
            let value = args.first().ok_or("dtcs:ceil requires one argument")?;
            match value.as_decimal() {
                Some(v) => Ok(RuntimeValue::Decimal(v.ceil())),
                None if value.is_null() => Ok(RuntimeValue::Null),
                None => Err("dtcs:ceil requires numeric".into()),
            }
        }
        "dtcs:power" => {
            if args.len() != 2 {
                return Err("dtcs:power requires two arguments".into());
            }
            match (args[0].as_decimal(), args[1].as_decimal()) {
                (Some(a), Some(b)) => Ok(RuntimeValue::Decimal(a.powf(b))),
                _ if args[0].is_null() || args[1].is_null() => Ok(RuntimeValue::Null),
                _ => Err("dtcs:power requires numeric arguments".into()),
            }
        }
        "dtcs:sqrt" => {
            let value = args.first().ok_or("dtcs:sqrt requires one argument")?;
            match value.as_decimal() {
                Some(v) if v < 0.0 => Err("dtcs:sqrt of negative".into()),
                Some(v) => Ok(RuntimeValue::Decimal(v.sqrt())),
                None if value.is_null() => Ok(RuntimeValue::Null),
                None => Err("dtcs:sqrt requires numeric".into()),
            }
        }
        "dtcs:least" => call_function("dtcs:min", args),
        "dtcs:greatest" => call_function("dtcs:max", args),
        "dtcs:count_all" | "dtcs:count" | "dtcs:count_distinct" | "dtcs:sum" | "dtcs:average" => {
            Err(format!(
                "{callee} is an aggregate function and must be used with dtcs:aggregate/group"
            ))
        }
        "dtcs:current_date" => {
            // Deterministic fixture clock for the reference runtime (run-stable within a process).
            Ok(RuntimeValue::Date(reference_clock_date()))
        }
        "dtcs:current_timestamp" => Ok(RuntimeValue::DateTime(reference_clock_timestamp())),
        "dtcs:date_add" => {
            if args.len() < 2 {
                return Err("dtcs:date_add requires date and integer amount".into());
            }
            let base = args[0]
                .as_str()
                .ok_or("dtcs:date_add first argument must be date/datetime string")?;
            let amount = args[1]
                .as_integer()
                .ok_or("dtcs:date_add second argument must be integer")?;
            let unit = args.get(2).and_then(RuntimeValue::as_str).unwrap_or("day");
            let shifted = shift_iso_date_unit(base, amount, unit)?;
            let is_datetime = matches!(args[0], RuntimeValue::DateTime(_)) || base.contains('T');
            Ok(if is_datetime {
                RuntimeValue::DateTime(if shifted.contains('T') {
                    shifted
                } else {
                    // Preserve original time-of-day when shifting by calendar units.
                    let (_, hour, minute, second, offset) = parse_datetime_parts(base)?;
                    format_datetime_with_offset(&shifted, hour, minute, second, offset)
                })
            } else {
                RuntimeValue::Date(shifted)
            })
        }
        "dtcs:date_diff" => {
            if args.len() < 2 {
                return Err("dtcs:date_diff requires two date/datetime arguments".into());
            }
            let left = args[0]
                .as_str()
                .ok_or("dtcs:date_diff requires date strings")?;
            let right = args[1]
                .as_str()
                .ok_or("dtcs:date_diff requires date strings")?;
            let unit = args.get(2).and_then(RuntimeValue::as_str).unwrap_or("day");
            Ok(RuntimeValue::Integer(diff_iso_dates_unit(
                left, right, unit,
            )?))
        }
        "dtcs:date_trunc" => {
            if args.len() != 2 {
                return Err("dtcs:date_trunc requires value and unit".into());
            }
            let value = args[0]
                .as_str()
                .ok_or("dtcs:date_trunc requires date/datetime")?;
            let unit = args[1]
                .as_str()
                .ok_or("dtcs:date_trunc unit must be string")?;
            Ok(RuntimeValue::DateTime(trunc_iso_datetime(value, unit)?))
        }
        "dtcs:extract" | "dtcs:date_part" => {
            if args.len() != 2 {
                return Err("dtcs:extract requires unit and date/datetime".into());
            }
            // Accept extract(unit, value) or extract(value, unit).
            let (unit, value) = if args[0].as_str().is_some_and(is_date_unit) {
                (
                    args[0].as_str().unwrap(),
                    args[1]
                        .as_str()
                        .ok_or("dtcs:extract value must be date string")?,
                )
            } else {
                (
                    args[1].as_str().ok_or("dtcs:extract unit must be string")?,
                    args[0]
                        .as_str()
                        .ok_or("dtcs:extract value must be date string")?,
                )
            };
            Ok(RuntimeValue::Integer(extract_date_part(value, unit)?))
        }
        "dtcs:at_timezone" => {
            if args.len() != 2 {
                return Err("dtcs:at_timezone requires datetime and fixed offset".into());
            }
            let value = args[0]
                .as_str()
                .ok_or("dtcs:at_timezone requires datetime string")?;
            let offset = args[1]
                .as_str()
                .ok_or("dtcs:at_timezone offset must be string like +00:00")?;
            Ok(RuntimeValue::DateTime(apply_fixed_offset(value, offset)?))
        }
        "dtcs:row_number" | "dtcs:rank" | "dtcs:dense_rank" | "dtcs:lag" | "dtcs:lead"
        | "dtcs:first_value" | "dtcs:last_value" | "dtcs:ntile" | "dtcs:percent_rank"
        | "dtcs:cume_dist" | "dtcs:nth_value" => Err(format!(
            "{callee} must be used with dtcs:window (not as a scalar call)"
        )),
        other => Err(format!("unsupported function '{other}'")),
    }
}

fn reference_clock_date() -> String {
    // Fixed reference clock so conformance / tests are deterministic.
    "2026-01-01".into()
}

fn reference_clock_timestamp() -> String {
    "2026-01-01T00:00:00Z".into()
}

fn parse_ymd(value: &str) -> Result<(i32, u32, u32), String> {
    let date = value.split('T').next().unwrap_or(value);
    let parts: Vec<_> = date.split('-').collect();
    if parts.len() != 3 {
        return Err(format!("invalid date '{value}'"));
    }
    let y: i32 = parts[0]
        .parse()
        .map_err(|_| format!("invalid year in '{value}'"))?;
    let m: u32 = parts[1]
        .parse()
        .map_err(|_| format!("invalid month in '{value}'"))?;
    let d: u32 = parts[2]
        .parse()
        .map_err(|_| format!("invalid day in '{value}'"))?;
    Ok((y, m, d))
}

fn days_from_civil(y: i32, m: u32, d: u32) -> i64 {
    // Howard Hinnant civil_from_days algorithm (proleptic Gregorian).
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = (y - era * 400) as u64;
    let m = m as i64;
    let d = d as i64;
    let mp = if m > 2 { m - 3 } else { m + 9 };
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe as i64 * 365 + yoe as i64 / 4 - yoe as i64 / 100 + doy;
    era as i64 * 146_097 + doe - 719_468
}

fn civil_from_days(z: i64) -> (i32, u32, u32) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
    let y = (yoe + era * 400) as i32;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

fn shift_iso_date(value: &str, days: i64) -> Result<String, String> {
    let (y, m, d) = parse_ymd(value)?;
    let (ny, nm, nd) = civil_from_days(days_from_civil(y, m, d) + days);
    Ok(format!("{ny:04}-{nm:02}-{nd:02}"))
}

fn diff_iso_dates(left: &str, right: &str) -> Result<i64, String> {
    let (ly, lm, ld) = parse_ymd(left)?;
    let (ry, rm, rd) = parse_ymd(right)?;
    Ok(days_from_civil(ly, lm, ld) - days_from_civil(ry, rm, rd))
}

fn shift_iso_date_unit(value: &str, amount: i64, unit: &str) -> Result<String, String> {
    match unit {
        "day" | "days" => shift_iso_date(value, amount),
        "month" | "months" => {
            let (y, m, d) = parse_ymd(value)?;
            let total = y as i64 * 12 + (m as i64 - 1) + amount;
            let ny = total.div_euclid(12) as i32;
            let nm = (total.rem_euclid(12) as u32) + 1;
            let max_day = days_in_month(ny, nm);
            let nd = d.min(max_day);
            Ok(format!("{ny:04}-{nm:02}-{nd:02}"))
        }
        "year" | "years" => {
            let (y, m, d) = parse_ymd(value)?;
            let ny = y + amount as i32;
            let max_day = days_in_month(ny, m);
            let nd = d.min(max_day);
            Ok(format!("{ny:04}-{m:02}-{nd:02}"))
        }
        "hour" | "hours" | "minute" | "minutes" => {
            let (date, hour, minute, second, _) = parse_datetime_parts(value)?;
            let (y, m, d) = parse_ymd(&date)?;
            let mut total_minutes =
                days_from_civil(y, m, d) * 24 * 60 + hour as i64 * 60 + minute as i64;
            match unit {
                "hour" | "hours" => total_minutes += amount * 60,
                _ => total_minutes += amount,
            }
            let day = total_minutes.div_euclid(24 * 60);
            let rem = total_minutes.rem_euclid(24 * 60);
            let nh = (rem / 60) as u32;
            let nm = (rem % 60) as u32;
            let (ny, nmo, nd) = civil_from_days(day);
            Ok(format!(
                "{ny:04}-{nmo:02}-{nd:02}T{nh:02}:{nm:02}:{second:02}Z"
            ))
        }
        other => Err(format!(
            "dtcs:date_add unsupported unit '{other}' (supports day/month/year/hour/minute)"
        )),
    }
}

fn diff_iso_dates_unit(left: &str, right: &str, unit: &str) -> Result<i64, String> {
    match unit {
        "day" | "days" => diff_iso_dates(left, right),
        "month" | "months" => {
            let (ly, lm, _) = parse_ymd(left)?;
            let (ry, rm, _) = parse_ymd(right)?;
            Ok((ly as i64 * 12 + lm as i64) - (ry as i64 * 12 + rm as i64))
        }
        "year" | "years" => {
            let (ly, _, _) = parse_ymd(left)?;
            let (ry, _, _) = parse_ymd(right)?;
            Ok((ly - ry) as i64)
        }
        "hour" | "hours" | "minute" | "minutes" => {
            let left_minutes = datetime_to_utc_minutes(left)?;
            let right_minutes = datetime_to_utc_minutes(right)?;
            let delta = left_minutes - right_minutes;
            Ok(if matches!(unit, "hour" | "hours") {
                delta / 60
            } else {
                delta
            })
        }
        other => Err(format!(
            "dtcs:date_diff unsupported unit '{other}' (supports day/month/year/hour/minute)"
        )),
    }
}

fn datetime_to_utc_minutes(value: &str) -> Result<i64, String> {
    let (date, hour, minute, _second, offset) = parse_datetime_parts(value)?;
    let (y, m, d) = parse_ymd(&date)?;
    Ok(days_from_civil(y, m, d) * 24 * 60 + hour as i64 * 60 + minute as i64 - offset as i64)
}

fn format_datetime_with_offset(
    date: &str,
    hour: u32,
    minute: u32,
    second: u32,
    offset_minutes: i32,
) -> String {
    if offset_minutes == 0 {
        return format!("{date}T{hour:02}:{minute:02}:{second:02}Z");
    }
    let sign = if offset_minutes >= 0 { '+' } else { '-' };
    let abs = offset_minutes.abs();
    format!(
        "{date}T{hour:02}:{minute:02}:{second:02}{sign}{:02}:{:02}",
        abs / 60,
        abs % 60
    )
}

fn days_in_month(y: i32, m: u32) -> u32 {
    match m {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap(y) => 29,
        2 => 28,
        _ => 30,
    }
}

fn is_leap(y: i32) -> bool {
    (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0)
}

fn is_date_unit(unit: &str) -> bool {
    matches!(
        unit,
        "year"
            | "years"
            | "month"
            | "months"
            | "day"
            | "days"
            | "hour"
            | "hours"
            | "minute"
            | "minutes"
            | "second"
            | "seconds"
    )
}

fn parse_datetime_parts(value: &str) -> Result<(String, u32, u32, u32, i32), String> {
    let (date, rest) = match value.split_once('T') {
        Some((d, r)) => (d.to_string(), r),
        None => return Ok((value.to_string(), 0, 0, 0, 0)),
    };
    let rest = rest.trim_end_matches('Z');
    let (time, offset_minutes) = if let Some((t, off)) = rest.split_once('+') {
        (t, parse_offset_minutes(&format!("+{off}"))?)
    } else if let Some((t, off)) = rest.split_once('-') {
        // Ambiguous with date; only treat as offset when time already has ':'
        if t.contains(':') {
            (t, parse_offset_minutes(&format!("-{off}"))?)
        } else {
            (rest, 0)
        }
    } else {
        (rest, 0)
    };
    let parts: Vec<_> = time.split(':').collect();
    if parts.len() < 2 {
        return Err(format!("invalid datetime '{value}'"));
    }
    let hour: u32 = parts[0]
        .parse()
        .map_err(|_| format!("invalid hour in '{value}'"))?;
    let minute: u32 = parts[1]
        .parse()
        .map_err(|_| format!("invalid minute in '{value}'"))?;
    let second: u32 = parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);
    Ok((date, hour, minute, second, offset_minutes))
}

fn parse_offset_minutes(offset: &str) -> Result<i32, String> {
    let trimmed = offset.trim();
    if trimmed == "Z" || trimmed == "+00:00" || trimmed == "-00:00" || trimmed.is_empty() {
        return Ok(0);
    }
    let sign = if trimmed.starts_with('-') { -1 } else { 1 };
    let body = trimmed.trim_start_matches(['+', '-']);
    let parts: Vec<_> = body.split(':').collect();
    if parts.len() != 2 {
        return Err(format!(
            "unsupported timezone '{offset}' (reference supports fixed offsets like +00:00)"
        ));
    }
    let hours: i32 = parts[0]
        .parse()
        .map_err(|_| format!("invalid timezone offset '{offset}'"))?;
    let minutes: i32 = parts[1]
        .parse()
        .map_err(|_| format!("invalid timezone offset '{offset}'"))?;
    Ok(sign * (hours * 60 + minutes))
}

fn trunc_iso_datetime(value: &str, unit: &str) -> Result<String, String> {
    let (date, hour, minute, second, _) = parse_datetime_parts(value)?;
    let (y, m, d) = parse_ymd(&date)?;
    Ok(match unit {
        "year" | "years" => format!("{y:04}-01-01T00:00:00Z"),
        "month" | "months" => format!("{y:04}-{m:02}-01T00:00:00Z"),
        "day" | "days" => format!("{y:04}-{m:02}-{d:02}T00:00:00Z"),
        "hour" | "hours" => format!("{y:04}-{m:02}-{d:02}T{hour:02}:00:00Z"),
        "minute" | "minutes" => format!("{y:04}-{m:02}-{d:02}T{hour:02}:{minute:02}:00Z"),
        "second" | "seconds" => {
            format!("{y:04}-{m:02}-{d:02}T{hour:02}:{minute:02}:{second:02}Z")
        }
        other => Err(format!("dtcs:date_trunc unsupported unit '{other}'"))?,
    })
}

fn extract_date_part(value: &str, unit: &str) -> Result<i64, String> {
    let (date, hour, minute, second, _) = parse_datetime_parts(value)?;
    let (y, m, d) = parse_ymd(&date)?;
    Ok(match unit {
        "year" | "years" => y as i64,
        "month" | "months" => m as i64,
        "day" | "days" => d as i64,
        "hour" | "hours" => hour as i64,
        "minute" | "minutes" => minute as i64,
        "second" | "seconds" => second as i64,
        other => return Err(format!("dtcs:extract unsupported unit '{other}'")),
    })
}

fn apply_fixed_offset(value: &str, offset: &str) -> Result<String, String> {
    let offset_minutes = parse_offset_minutes(offset)?;
    let (date, hour, minute, second, current_offset) = parse_datetime_parts(value)?;
    let (y, m, d) = parse_ymd(&date)?;
    let mut total_minutes = days_from_civil(y, m, d) * 24 * 60 + hour as i64 * 60 + minute as i64
        - current_offset as i64;
    total_minutes += offset_minutes as i64;
    let day = total_minutes.div_euclid(24 * 60);
    let rem = total_minutes.rem_euclid(24 * 60);
    let nh = (rem / 60) as u32;
    let nm = (rem % 60) as u32;
    let (ny, nmo, nd) = civil_from_days(day);
    let sign = if offset_minutes >= 0 { '+' } else { '-' };
    let abs = offset_minutes.abs();
    Ok(format!(
        "{ny:04}-{nmo:02}-{nd:02}T{nh:02}:{nm:02}:{second:02}{sign}{:02}:{:02}",
        abs / 60,
        abs % 60
    ))
}

fn eval_between(
    value: &RuntimeValue,
    lo: &RuntimeValue,
    hi: &RuntimeValue,
) -> Result<RuntimeValue, String> {
    if value.is_null()
        || value.is_missing()
        || lo.is_null()
        || lo.is_missing()
        || hi.is_null()
        || hi.is_missing()
    {
        return Ok(RuntimeValue::Null);
    }
    if value.is_invalid() || lo.is_invalid() || hi.is_invalid() {
        return Ok(RuntimeValue::Null);
    }
    let ge_lo = compare_ordered_values(value, lo)?;
    let le_hi = compare_ordered_values(hi, value)?;
    Ok(RuntimeValue::Boolean(
        ge_lo != std::cmp::Ordering::Less && le_hi != std::cmp::Ordering::Less,
    ))
}

fn compare_ordered_values(
    left: &RuntimeValue,
    right: &RuntimeValue,
) -> Result<std::cmp::Ordering, String> {
    match (left, right) {
        (RuntimeValue::Integer(a), RuntimeValue::Integer(b)) => Ok(a.cmp(b)),
        (RuntimeValue::Decimal(a), RuntimeValue::Decimal(b)) => a
            .partial_cmp(b)
            .ok_or_else(|| "decimal comparison failed".to_string()),
        (RuntimeValue::Integer(a), RuntimeValue::Decimal(b)) => (*a as f64)
            .partial_cmp(b)
            .ok_or_else(|| "decimal comparison failed".to_string()),
        (RuntimeValue::Decimal(a), RuntimeValue::Integer(b)) => a
            .partial_cmp(&(*b as f64))
            .ok_or_else(|| "decimal comparison failed".to_string()),
        (RuntimeValue::String(a), RuntimeValue::String(b)) => Ok(a.cmp(b)),
        (RuntimeValue::Date(a), RuntimeValue::Date(b))
        | (RuntimeValue::DateTime(a), RuntimeValue::DateTime(b))
        | (RuntimeValue::Date(a), RuntimeValue::DateTime(b))
        | (RuntimeValue::DateTime(a), RuntimeValue::Date(b)) => Ok(a.cmp(b)),
        _ => Err("between comparison type mismatch".into()),
    }
}

fn eval_field_access(
    container: &RuntimeValue,
    field: &RuntimeValue,
) -> Result<RuntimeValue, String> {
    let name = field.as_str().ok_or("dtcs:field name must be a string")?;
    match container {
        RuntimeValue::Null | RuntimeValue::Missing(_) => Ok(RuntimeValue::Null),
        RuntimeValue::Map(map) => Ok(map.get(name).cloned().unwrap_or_else(RuntimeValue::missing)),
        other => Err(format!("dtcs:field requires map/object, got {other:?}")),
    }
}

fn eval_index_access(
    container: &RuntimeValue,
    index: &RuntimeValue,
    null_on_oob: bool,
) -> Result<RuntimeValue, String> {
    match container {
        RuntimeValue::Null | RuntimeValue::Missing(_) => Ok(RuntimeValue::Null),
        RuntimeValue::List(items) => {
            let idx = index.as_integer().ok_or("list index must be an integer")?;
            if idx < 0 {
                return if null_on_oob {
                    Ok(RuntimeValue::Null)
                } else {
                    Err("list index must be non-negative".into())
                };
            }
            match items.get(idx as usize) {
                Some(v) => Ok(v.clone()),
                None if null_on_oob => Ok(RuntimeValue::Null),
                None => Err(format!("list index {idx} out of bounds")),
            }
        }
        RuntimeValue::Map(map) => {
            let key = index.as_str().ok_or("map index/key must be a string")?;
            match map.get(key) {
                Some(v) => Ok(v.clone()),
                None if null_on_oob => Ok(RuntimeValue::Null),
                None => Ok(RuntimeValue::missing()),
            }
        }
        other => Err(format!(
            "index/element_at requires list or map, got {other:?}"
        )),
    }
}

fn string_arg<'a>(args: &'a [RuntimeValue], index: usize, callee: &str) -> Result<&'a str, String> {
    args.get(index)
        .ok_or_else(|| format!("{callee} requires argument {}", index + 1))
        .and_then(|value| string_value(value, callee))
}

fn cast_runtime_value(value: &RuntimeValue, target: &str) -> Result<RuntimeValue, String> {
    if matches!(value, RuntimeValue::Null | RuntimeValue::Missing(_)) {
        return Ok(RuntimeValue::Null);
    }
    match target {
        "string" => Ok(RuntimeValue::String(match value {
            RuntimeValue::String(text) => text.clone(),
            RuntimeValue::Integer(number) => number.to_string(),
            RuntimeValue::Decimal(number) => number.to_string(),
            RuntimeValue::Boolean(value) => value.to_string(),
            _ => return Err("cast to string unsupported for value".into()),
        })),
        "integer" => value
            .as_integer()
            .map(RuntimeValue::Integer)
            .ok_or_else(|| "cast to integer failed".into()),
        "decimal" => value
            .as_decimal()
            .map(RuntimeValue::Decimal)
            .ok_or_else(|| "cast to decimal failed".into()),
        "boolean" => match value {
            RuntimeValue::Boolean(value) => Ok(RuntimeValue::Boolean(*value)),
            RuntimeValue::String(text) => match text.to_ascii_lowercase().as_str() {
                "true" | "1" | "yes" => Ok(RuntimeValue::Boolean(true)),
                "false" | "0" | "no" => Ok(RuntimeValue::Boolean(false)),
                _ => Err("cast to boolean failed".into()),
            },
            _ => Err("cast to boolean failed".into()),
        },
        other => Err(format!("unsupported cast target '{other}'")),
    }
}

fn parse_timezone(value: &str) -> Result<Tz, String> {
    value
        .parse::<Tz>()
        .map_err(|_| format!("unknown IANA timezone '{value}'"))
}

fn unit_random(seed: u64) -> f64 {
    super::prng::XorShift64Star::new(seed).next_f64()
}

fn run_id() -> &'static String {
    static RUN_ID: OnceLock<String> = OnceLock::new();
    RUN_ID.get_or_init(|| uuid::Uuid::new_v4().to_string())
}

fn run_timestamp() -> &'static String {
    static RUN_TIMESTAMP: OnceLock<String> = OnceLock::new();
    RUN_TIMESTAMP.get_or_init(|| Utc::now().to_rfc3339())
}

fn string_value<'a>(value: &'a RuntimeValue, callee: &str) -> Result<&'a str, String> {
    match value {
        RuntimeValue::String(text) => Ok(text),
        RuntimeValue::Null | RuntimeValue::Missing(_) => {
            Err(format!("{callee} requires present string arguments"))
        }
        other => Err(format!("{callee} requires string arguments, got {other:?}")),
    }
}

fn stats_mode_from_string(value: &str) -> Result<bool, String> {
    match value {
        "population" => Ok(true),
        "sample" => Ok(false),
        other => Err(format!(
            "unsupported statistics mode '{other}'; expected sample|population"
        )),
    }
}

fn stats_numbers_and_mode(args: &[RuntimeValue]) -> Result<(Vec<f64>, bool), String> {
    let mut population = false;
    let mut nums = Vec::new();
    for (index, arg) in args.iter().enumerate() {
        if index + 1 == args.len() {
            if let RuntimeValue::String(mode) = arg {
                population = stats_mode_from_string(mode)?;
                continue;
            }
        }
        if let Some(n) = arg.as_decimal() {
            nums.push(n);
        }
    }
    Ok((nums, population))
}

fn covariance_args(
    args: &[RuntimeValue],
) -> Result<(&Vec<RuntimeValue>, &Vec<RuntimeValue>, bool), String> {
    if !(2..=3).contains(&args.len()) {
        return Err("dtcs:covariance requires two lists and optional mode".into());
    }
    let (RuntimeValue::List(xs), RuntimeValue::List(ys)) = (&args[0], &args[1]) else {
        return Err("dtcs:covariance requires two lists".into());
    };
    let population = match args.get(2) {
        None => false,
        Some(RuntimeValue::String(mode)) => stats_mode_from_string(mode)?,
        Some(_) => {
            return Err("dtcs:covariance mode must be 'sample' or 'population'".into());
        }
    };
    Ok((xs, ys, population))
}

type MapConcatParts<'a> = (Vec<IndexMap<String, RuntimeValue>>, Option<&'a str>);

fn split_map_concat_args(args: &[RuntimeValue]) -> Result<MapConcatParts<'_>, String> {
    if args.is_empty() {
        return Ok((Vec::new(), None));
    }
    let last_is_policy = matches!(
        args.last(),
        Some(RuntimeValue::String(s)) if matches!(s.as_str(), "error" | "left" | "right")
    );
    let map_count = if last_is_policy {
        args.len() - 1
    } else {
        args.len()
    };
    if map_count >= 2 && !last_is_policy {
        return Err("map_concat requires collisionPolicy error|left|right".into());
    }
    let mut maps = Vec::with_capacity(map_count);
    for value in &args[..map_count] {
        let RuntimeValue::Map(map) = value else {
            return Err("dtcs:map_concat requires maps".into());
        };
        maps.push(map.clone());
    }
    let policy = if last_is_policy {
        args.last().and_then(RuntimeValue::as_str)
    } else {
        None
    };
    Ok((maps, policy))
}

fn regex_extract_all_advancing(
    regex: &regex::Regex,
    text: &str,
    group: usize,
) -> Vec<RuntimeValue> {
    let mut out = Vec::new();
    let mut locs = regex.capture_locations();
    let mut pos = 0;
    while pos <= text.len() {
        if regex.captures_read_at(&mut locs, text, pos).is_none() {
            break;
        }
        if let Some((start, end)) = locs.get(group) {
            out.push(RuntimeValue::String(text[start..end].into()));
        }
        let Some((match_start, match_end)) = locs.get(0) else {
            break;
        };
        let next = if match_end == match_start {
            if match_end >= text.len() {
                break;
            }
            match_end
                + text[match_end..]
                    .chars()
                    .next()
                    .map(|c| c.len_utf8())
                    .unwrap_or(1)
        } else {
            match_end
        };
        if next <= pos {
            break;
        }
        pos = next;
    }
    out
}

fn regex_split_advancing(regex: &regex::Regex, text: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut last = 0usize;
    let mut pos = 0usize;
    while pos <= text.len() {
        let Some(found) = regex.find_at(text, pos) else {
            break;
        };
        parts.push(text[last..found.start()].to_string());
        if found.end() > found.start() {
            last = found.end();
            pos = found.end();
        } else {
            last = found.start();
            if found.start() >= text.len() {
                break;
            }
            pos = found.start()
                + text[found.start()..]
                    .chars()
                    .next()
                    .map(|c| c.len_utf8())
                    .unwrap_or(1);
        }
        if pos <= found.start() && found.end() == found.start() {
            break;
        }
    }
    parts.push(text[last.min(text.len())..].to_string());
    parts
}
