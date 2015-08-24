use protobuf::{CodedInputStream, Message, ProtobufResult};


struct MessageBuilder<'a, T: Message> {
    msg: T,
    lastPos: u32,
    is: CodedInputStream<'a>,
}

impl <'a, T: Message> MessageBuilder<'a, T> {

    pub fn new<'b>(msg: T, bytes: &'b [u8]) -> MessageBuilder<'b, T> {
        MessageBuilder {
            msg: msg,
            lastPos: 0,
            is: CodedInputStream::from_bytes(bytes),
        }
    }

    pub fn get(&self) -> Option<T> {
        match self.msg.is_initialized() {
            true => Some(self.msg),
            false => None
        }
    }

    pub fn read<'b>(&mut self, bytes: &'b mut [u8]) -> ProtobufResult<(Option<T>, u32)> {
        try!(self.is.read(bytes));
        match self.msg.merge_from(&mut self.is) {
            Err(e) => {
                Err(e)
            },
            _ => {
                let bytesRead = self.is.pos() - self.lastPos;
                self.lastPos = self.is.pos();
                match self.msg.is_initialized() {
                    true => Ok((Some(self.msg), bytesRead)),
                    false => Ok((None, bytesRead))
                }
            }
        }
    }

}
