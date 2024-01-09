use std::fmt;

struct NodeReport {
	code: u16,
	mnemonic: String,
	description: String
}

trait ToString for NodeReport {
	fn to_string(&self) -> String {
		format!("#{} {}", self.code, self.description)
	}
	fn to_string_for_packet(&self, counter: u16) -> String {
		format!("#{} [{}] {}", self.code, counter, self.description)
	}
	fn to_packet(&self, counter: u16) -> [char; 64] {
		let packet_str = self.to_string_for_packet(counter);
		let field_data = Vec::with_capacity(64);
		field_data.push(packet_str).resize_with(64, 0)
	}
}


pub const NodeReports = [
	NodeReport { 
		code: 0x0000,
		mnemonic: "RcDebug",
		description: "Booted in debug mode (Only used in development)"
	}
]