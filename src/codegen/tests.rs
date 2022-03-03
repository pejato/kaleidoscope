use std::ffi::CString;

use crate::ast::{Expr, ExprKind};

use super::*;
use indoc::indoc;
use pretty_assertions::assert_eq;

fn make_generator(context: &Context) -> CodeGen {
    let module = context.create_module("Test");
    let builder = context.create_builder();
    CodeGen {
        context,
        builder,
        module,
        named_values: HashMap::new(),
        current_function: None,
    }
}

#[test]
fn test_codegen_number() {
    let context = Context::create();
    let generator = make_generator(&context);
    let result = generator.codegen_number(32.0);

    assert_eq!(result.get_constant().unwrap().0, 32.0);
}

#[test]
fn test_codegen_var() {
    let context = Context::create();
    let mut generator = make_generator(&context);

    // This is kinda weird but all we want to do is verify that this method fetches the AnyValueEnum corresponding to
    // the passed variable name.
    let any_value = context.f64_type().const_float(30.0).as_any_value_enum();
    generator.named_values.insert("x".to_owned(), any_value);

    let result = generator.codegen_variable("x").unwrap();
    assert_eq!(result, any_value);
}

#[test]
fn test_codegen_bin_plus() {
    let context = Context::create();
    let mut generator = make_generator(&context);

    let lhs = Expr {
        kind: ExprKind::Number(14.0),
    };
    let rhs = Expr {
        kind: ExprKind::Number(41.0),
    };

    let result = generator.codegen_binary('+', &lhs, &rhs).unwrap();
    assert_eq!(result.get_constant().unwrap().0, 55.0);
}

#[test]
fn test_codegen_bin_minus() {
    let context = Context::create();
    let mut generator = make_generator(&context);

    let lhs = Expr {
        kind: ExprKind::Number(14.0),
    };
    let rhs = Expr {
        kind: ExprKind::Number(41.0),
    };

    let result = generator.codegen_binary('-', &lhs, &rhs).unwrap();
    assert_eq!(result.get_constant().unwrap().0, -27.0);
}

#[test]
fn test_codegen_bin_mult() {
    let context = Context::create();
    let mut generator = make_generator(&context);

    let lhs = Expr {
        kind: ExprKind::Number(14.0),
    };
    let rhs = Expr {
        kind: ExprKind::Number(41.0),
    };

    let result = generator.codegen_binary('*', &lhs, &rhs).unwrap();
    assert_eq!(result.get_constant().unwrap().0, 574.0);
}

#[test]
fn test_codegen_bin_less_than_true() {
    let context = Context::create();
    let mut generator = make_generator(&context);

    let lhs = Expr {
        kind: ExprKind::Number(14.0),
    };
    let rhs = Expr {
        kind: ExprKind::Number(41.0),
    };

    let result = generator.codegen_binary('<', &lhs, &rhs).unwrap();
    assert_eq!(result.get_constant().unwrap().0, 1.0);
}

#[test]
fn test_codegen_bin_less_than_false() {
    let context = Context::create();
    let mut generator = make_generator(&context);

    let lhs = Expr {
        kind: ExprKind::Number(41.0),
    };
    let rhs = Expr {
        kind: ExprKind::Number(41.0),
    };

    let result = generator.codegen_binary('<', &lhs, &rhs).unwrap();
    assert_eq!(result.get_constant().unwrap().0, 0.0);
}

#[test]
fn test_codegen_bin_unknown() {
    let context = Context::create();
    let mut generator = make_generator(&context);

    let lhs = Expr {
        kind: ExprKind::Number(41.0),
    };
    let rhs = Expr {
        kind: ExprKind::Number(41.0),
    };

    let result = generator.codegen_binary('#', &lhs, &rhs);
    assert_eq!(result, None);
}

#[test]
fn test_codegen_call_with_gened_prototype() {
    let context = Context::create();
    let mut generator = make_generator(&context);

    assert!(generator
        .codegen_prototype(&["x".into(), "y".into()], "flint")
        .is_some());

    let callee = "flint";
    let args = [
        Expr {
            kind: ExprKind::Number(67.0),
        },
        Expr {
            kind: ExprKind::Number(67.0),
        },
    ];
    let result = generator.codegen_call(callee, &args);

    let result_as_string = result.map(|r| r.print_to_string().to_string()).unwrap();
    let expected =
        "%call_tmp = call addrspace(0) double @flint(double 6.700000e+01, double 6.700000e+01)";
    assert_eq!(result_as_string.trim(), expected);
}

#[test]
fn test_codegen_call_with_gened_function() {
    let context = Context::create();
    let mut generator = make_generator(&context);

    let prototype = Expr {
        kind: ExprKind::Prototype {
            name: "Juwan".into(),
            args: vec!["x".into(), "y".into()],
        },
    };
    let body = Expr {
        kind: ExprKind::Binary {
            operator: '+',
            lhs: Expr {
                kind: ExprKind::Variable { name: "x".into() },
            }
            .into(),
            rhs: Expr {
                kind: ExprKind::Variable { name: "y".into() },
            }
            .into(),
        },
    };

    assert!(generator.codegen_function(&prototype, &body).is_some());

    let callee = "Juwan";
    let args = [
        Expr {
            kind: ExprKind::Number(67.0),
        },
        Expr {
            kind: ExprKind::Number(67.0),
        },
    ];
    let result = generator.codegen_call(callee, &args);
    if let Some(fv) = result {
        fv.print_to_stderr()
    }
    let result_as_string = result.map(|r| r.print_to_string().to_string()).unwrap();
    let expected = "%call_tmp = call double @Juwan(double 6.700000e+01, double 6.700000e+01)";
    assert_eq!(result_as_string.trim(), expected);
}

#[test]
fn test_codegen_fn_prototype() {
    let context = Context::create();
    let generator = make_generator(&context);

    let args = vec!["x".into(), "y".into()];
    let name = "Moonlight";
    let result = generator.codegen_prototype(&args, name).unwrap();

    assert_eq!(result.get_params().len(), 2);
    assert!(result.get_type().get_return_type().unwrap().is_float_type());
    assert!(result
        .get_type()
        .get_param_types()
        .into_iter()
        .all(|ty| ty.is_float_type()));

    let param_names: Vec<String> = result
        .get_param_iter()
        .filter_map(|param| {
            param
                .into_float_value()
                .get_name()
                .to_str()
                .ok()
                .map(|n| n.to_string())
        })
        .collect();

    assert_eq!(param_names, vec!["x".to_string(), "y".to_string()]);
    assert_eq!(
        result.get_name().to_owned(),
        CString::new("Moonlight").unwrap()
    );
}

#[test]
fn test_codegen_function() {
    let context = Context::create();
    let mut generator = make_generator(&context);

    let prototype = Expr {
        kind: ExprKind::Prototype {
            name: "Juwan".into(),
            args: vec!["x".into(), "y".into()],
        },
    };
    let body = Expr {
        kind: ExprKind::Binary {
            operator: '+',
            lhs: Expr {
                kind: ExprKind::Variable { name: "x".into() },
            }
            .into(),
            rhs: Expr {
                kind: ExprKind::Variable { name: "y".into() },
            }
            .into(),
        },
    };

    let result = generator.codegen_function(&prototype, &body);
    assert!(result.is_some());
    let result = result.unwrap();
    let expected = indoc! {"
    define double @Juwan(double %x, double %y) {
    entry:
      %addtmp = fadd double %x, %y
      ret double %addtmp
    }
    "};

    // The textual output includes param types, names, and basic blocks so I think it's sufficient to assert on it
    assert_eq!(result.print_to_string().to_string(), expected);
}

#[test]
fn test_codegen_function_two_calls() {
    let context = Context::create();
    let mut generator = make_generator(&context);

    let juwan_proto = Expr {
        kind: ExprKind::Prototype {
            name: "Juwan".into(),
            args: vec!["x".into()],
        },
    };
    let juwan_body = Expr {
        kind: ExprKind::Binary {
            operator: '*',
            lhs: Expr {
                kind: ExprKind::Variable { name: "x".into() },
            }
            .into(),
            rhs: Expr {
                kind: ExprKind::Number(2.0),
            }
            .into(),
        },
    };

    let howard_proto = Expr {
        kind: ExprKind::Prototype {
            name: "Howard".into(),
            args: vec!["y".into()],
        },
    };
    let howard_body = Expr {
        kind: ExprKind::Binary {
            operator: '+',
            lhs: Expr {
                kind: ExprKind::Variable { name: "y".into() },
            }
            .into(),
            rhs: Expr {
                kind: ExprKind::Number(4.0),
            }
            .into(),
        },
    };
    assert!(generator
        .codegen_function(&juwan_proto, &juwan_body)
        .is_some());
    assert!(generator
        .codegen_function(&howard_proto, &howard_body)
        .is_some());

    let juwan_howard_proto = Expr {
        kind: ExprKind::Prototype {
            name: "JuwanHoward".into(),
            args: vec!["x".into(), "y".into()],
        },
    };
    let juwan_howard_body = Expr {
        kind: ExprKind::Binary {
            operator: '+',
            lhs: Expr {
                kind: ExprKind::Call {
                    callee: "Juwan".into(),
                    args: vec![Expr {
                        kind: ExprKind::Variable { name: "x".into() },
                    }],
                },
            }
            .into(),
            rhs: Expr {
                kind: ExprKind::Call {
                    callee: "Howard".into(),
                    args: vec![Expr {
                        kind: ExprKind::Variable { name: "y".into() },
                    }],
                },
            }
            .into(),
        },
    };

    let result = generator.codegen_function(&juwan_howard_proto, &juwan_howard_body);
    assert!(result.is_some());
    let result = result.unwrap();

    // We set the name of call's to call_tmp. This test specifically tests that
    // later calls don't clobber earlier call_tmp's
    let expected = indoc! {"
    define double @JuwanHoward(double %x, double %y) {
    entry:
      %call_tmp = call double @Juwan(double %x)
      %call_tmp1 = call double @Howard(double %y)
      %addtmp = fadd double %call_tmp, %call_tmp1
      ret double %addtmp
    }
    "};

    assert_eq!(result.print_to_string().to_string(), expected);
}

#[test]
fn test_codegen_if_then_else() {
    let context = Context::create();
    let mut generator = make_generator(&context);

    let result = generator
        .codegen(&Expr {
            kind: ExprKind::Function {
                prototype: Expr {
                    kind: ExprKind::Prototype {
                        name: "fib".into(),
                        args: vec!["x".into()],
                    },
                }
                .into(),
                body: Expr {
                    kind: ExprKind::If(IfVal {
                        if_boolish_test: Expr {
                            kind: ExprKind::Binary {
                                operator: '<',
                                lhs: Expr {
                                    kind: ExprKind::Variable { name: "x".into() },
                                }
                                .into(),
                                rhs: Expr {
                                    kind: ExprKind::Number(2.0),
                                }
                                .into(),
                            }
                            .into(),
                        }
                        .into(),
                        then: Expr {
                            kind: ExprKind::Call {
                                callee: "fib".into(),
                                args: vec![Expr {
                                    kind: ExprKind::Binary {
                                        operator: '-',
                                        lhs: Expr {
                                            kind: ExprKind::Variable { name: "x".into() },
                                        }
                                        .into(),
                                        rhs: Expr {
                                            kind: ExprKind::Number(1.0),
                                        }
                                        .into(),
                                    },
                                }
                                .into()],
                            },
                        }
                        .into(),
                        elves: Expr {
                            kind: ExprKind::Call {
                                callee: "fib".into(),
                                args: vec![Expr {
                                    kind: ExprKind::Binary {
                                        operator: '+',
                                        lhs: Expr {
                                            kind: ExprKind::Variable { name: "x".into() },
                                        }
                                        .into(),
                                        rhs: Expr {
                                            kind: ExprKind::Number(1.0),
                                        }
                                        .into(),
                                    },
                                }
                                .into()],
                            },
                        }
                        .into(),
                    }),
                }
                .into(),
            },
        })
        .unwrap();

    let result_string = result.print_to_string().to_string();
    let expected_string = indoc!(
        "
        define double @fib(double %x) {
        entry:
          %cmptmp = fcmp ult double %x, 2.000000e+00
          %booltmp = uitofp i1 %cmptmp to double
          %comp = fcmp one double %booltmp, 0.000000e+00
          br i1 %comp, label %then, label %else
        
        then:                                             ; preds = %entry
          %subtmp = fsub double %x, 1.000000e+00
          %call_tmp = call double @fib(double %subtmp)
          br label %cont
        
        else:                                             ; preds = %entry
          %addtmp = fadd double %x, 1.000000e+00
          %call_tmp1 = call double @fib(double %addtmp)
          br label %cont
        
        cont:                                             ; preds = %else, %then
          %iftmp = phi double [ %call_tmp, %then ], [ %call_tmp1, %else ]
          ret double %iftmp
        }"
    );
    assert_eq!(result_string.trim(), expected_string);
}
