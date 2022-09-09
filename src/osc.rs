  // Create a string showing all the packets.
  let mut packets_text = format!("Listening on port {}\nReceived packets:\n", OSC_PORT);

  for (addr, packet) in model.received_packets.iter().rev() {
      let x: Option<(i32, i32)> = if let osc::Packet::Message(m) = packet {
          match m.addr.as_str() {
              "/pose/position" => {
                  if let Some(args) = &m.args {
                      None
                  } else {
                      None
                  }
              }

              _ => None,
          }
      } else {
          None
      };

      //if m.addr.as_str() == "/pose/position" {

      // match packet {
      //     osc::Packet::Message(m) => matr,
      //     osc::Packet::Bundle(_) => todo!(),
      // }
  }

  for &(addr, ref packet) in model.received_packets.iter().rev() {
      packets_text.push_str(&format!("{}: {:?}\n", addr, packet));
  }



  for (packet, addr) in model.receiver.try_iter() {
    model.received_packets.push((addr, packet));
}

// We'll display 10 packets at a time, so remove any excess.
let max_packets = 10;
while model.received_packets.len() > max_packets {
    model.received_packets.remove(0);
}
