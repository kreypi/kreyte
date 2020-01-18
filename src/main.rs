// FIXME: Ugly i assume
pub mod libs { pub mod output { pub mod efixme; } }

// FIXME: How to export this as a crate so that any repository could use `efixme("msg");` ?

fn main() {
	// FIXME: Expecting `efixme("test");` only
	libs::output::efixme::efixme("test");
}
