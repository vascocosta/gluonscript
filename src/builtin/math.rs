use std::collections::HashMap;

use crate::runtime::{RuntimeError, Value};

pub fn module() -> Result<Value, RuntimeError> {
    let mut map = HashMap::new();

    map.insert("abs".to_string(), Value::BuiltinFunction(abs));
    map.insert("ceil".to_string(), Value::BuiltinFunction(ceil));
    map.insert("clamp".to_string(), Value::BuiltinFunction(clamp));
    map.insert("floor".to_string(), Value::BuiltinFunction(floor));
    map.insert("round".to_string(), Value::BuiltinFunction(round));

    Ok(Value::Record(map))
}

pub fn abs(args: Vec<Value>) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::Arity {
            expected: 1,
            got: args.len(),
        });
    }

    match args.first() {
        Some(Value::Float(number)) => Ok(Value::Float(number.abs())),
        Some(Value::Int(number)) => Ok(Value::Int(number.abs())),

        None => Err(RuntimeError::Arity {
            expected: 1,
            got: 0,
        }),

        other => Err(RuntimeError::TypeError {
            expected: "float/int",
            got: format!("{:?}", other),
        }),
    }
}

pub fn ceil(args: Vec<Value>) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::Arity {
            expected: 1,
            got: args.len(),
        });
    }

    match args.first() {
        Some(Value::Float(number)) => Ok(Value::Float(number.ceil())),

        None => Err(RuntimeError::Arity {
            expected: 1,
            got: 0,
        }),

        other => Err(RuntimeError::TypeError {
            expected: "float",
            got: format!("{:?}", other),
        }),
    }
}

pub fn clamp(args: Vec<Value>) -> Result<Value, RuntimeError> {
    if args.len() != 3 {
        return Err(RuntimeError::Arity {
            expected: 3,
            got: args.len(),
        });
    }

    match args.first() {
        Some(Value::Float(v)) => {
            let min = match args.iter().nth(1) {
                Some(Value::Float(min)) => min,

                Some(other) => {
                    return Err(RuntimeError::TypeError {
                        expected: "float",
                        got: format!("{:?}", other),
                    });
                }

                None => {
                    return Err(RuntimeError::Arity {
                        expected: 3,
                        got: 0,
                    });
                }
            };

            let max = match args.iter().nth(2) {
                Some(Value::Float(max)) => max,

                Some(other) => {
                    return Err(RuntimeError::TypeError {
                        expected: "float",
                        got: format!("{:?}", other),
                    });
                }

                None => {
                    return Err(RuntimeError::Arity {
                        expected: 3,
                        got: 0,
                    });
                }
            };

            return Ok(Value::Float(v.clamp(*min, *max)));
        }

        Some(Value::Int(v)) => {
            let min = match args.iter().nth(1) {
                Some(Value::Int(min)) => min,

                Some(other) => {
                    return Err(RuntimeError::TypeError {
                        expected: "int",
                        got: format!("{:?}", other),
                    });
                }

                None => {
                    return Err(RuntimeError::Arity {
                        expected: 3,
                        got: 0,
                    });
                }
            };

            let max = match args.iter().nth(2) {
                Some(Value::Int(max)) => max,

                Some(other) => {
                    return Err(RuntimeError::TypeError {
                        expected: "int",
                        got: format!("{:?}", other),
                    });
                }

                None => {
                    return Err(RuntimeError::Arity {
                        expected: 3,
                        got: 0,
                    });
                }
            };

            return Ok(Value::Int(*v.clamp(min, max)));
        }

        Some(other) => {
            return Err(RuntimeError::TypeError {
                expected: "float/int",
                got: format!("{:?}", other),
            });
        }

        None => {
            return Err(RuntimeError::Arity {
                expected: 3,
                got: 0,
            });
        }
    }
}

pub fn floor(args: Vec<Value>) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::Arity {
            expected: 1,
            got: args.len(),
        });
    }

    match args.first() {
        Some(Value::Float(number)) => Ok(Value::Float(number.floor())),

        None => Err(RuntimeError::Arity {
            expected: 1,
            got: 0,
        }),

        other => Err(RuntimeError::TypeError {
            expected: "float",
            got: format!("{:?}", other),
        }),
    }
}

pub fn round(args: Vec<Value>) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::Arity {
            expected: 1,
            got: args.len(),
        });
    }

    match args.first() {
        Some(Value::Float(number)) => Ok(Value::Float(number.round())),

        None => Err(RuntimeError::Arity {
            expected: 1,
            got: 0,
        }),

        other => Err(RuntimeError::TypeError {
            expected: "float",
            got: format!("{:?}", other),
        }),
    }
}
