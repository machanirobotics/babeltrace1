use babeltrace1::{ctf, Context, Error, Format};

pub fn main() -> Result<(), Error> {
    let mut ctx = Context::new()?;
    ctx.add_trace("examples/ctf-trace", Format::Ctf)?;

    for event in ctx.ctf_iter().unwrap() {
        if event.name() != "string" {
            continue;
        }

        let scope = event.get_top_level_scope(ctf::Scope::EventFields)?;
        let Some(field) = event.get_field(&scope, "str") else {
            println!("failed to get field 'str', skipping");
            continue;
        };

        println!(
            "name: {}, fields: 'str' => {}",
            event.name(),
            field.get_str()?,
        );
    }

    Ok(())
}
