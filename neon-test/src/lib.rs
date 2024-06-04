use neon::prelude::*;

fn fibonacci(n: i32) -> i32 {
    match n {
        1 | 2 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn js_fibonacci(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let n = cx.argument::<JsNumber>(0)?.value(&mut cx);

    Ok(cx.number(fibonacci(n as i32)))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("fibonacci", js_fibonacci)?;
    Ok(())
}
