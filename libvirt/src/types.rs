use byteorder::{BigEndian, ByteOrder};
use lib_heat_spec::h_type;
use lib_heat_spec::h_type::HType;

pub type VirtualAddress = u64;

pub struct VirtualPointer {
    pub location: VirtualAddress,
    pub data_type: HType,
}

#[derive(Clone, Debug)]
pub struct VirtualObject {
    pub data: Vec<u8>,
    pub data_type: HType,
}

impl PartialEq for VirtualObject {
    fn eq(&self, other: &Self) -> bool {
        return self.data_type == other.data_type && self.data == other.data;
    }
}

impl From<bool> for VirtualObject {
    fn from(boolean: bool) -> VirtualObject {
        let mut obj = VirtualObject::new_empty(HType::Bool);
        obj.set_bool(&boolean);
        return obj;
    }
}
impl From<u8> for VirtualObject {
    fn from(u8: u8) -> VirtualObject {
        let mut obj = VirtualObject::new_empty(HType::U8);
        obj.set_u8(&u8);
        return obj;
    }
}
impl From<u16> for VirtualObject {
    fn from(u16: u16) -> VirtualObject {
        let mut obj = VirtualObject::new_empty(HType::U16);
        obj.set_u16(&u16);
        return obj;
    }
}
impl From<u32> for VirtualObject {
    fn from(u32: u32) -> VirtualObject {
        let mut obj = VirtualObject::new_empty(HType::U32);
        obj.set_u32(&u32);
        return obj;
    }
}
impl From<u64> for VirtualObject {
    fn from(u64: u64) -> VirtualObject {
        let mut obj = VirtualObject::new_empty(HType::U64);
        obj.set_u64(&u64);
        return obj;
    }
}




impl VirtualObject {
    pub fn new(data: Vec<u8>, data_type: HType) -> VirtualObject {
        VirtualObject { data, data_type }
    }

    
    /// Create an VirtualObject with an empty `Vec<u8>` with the capacity required to hold the `HType`
    pub fn new_empty(data_type: HType) -> VirtualObject {
        let mut vec = Vec::<u8>::with_capacity(h_type::get_size(data_type));
        for _ in 0..h_type::get_size(data_type){
            vec.push(0u8);
        }

        VirtualObject {
            data: vec,
            data_type,
        }
    }

    /// Create an VirtualObject with max filled `Vec<u8>` for the given `HType`
    pub fn new_max(data_type: HType) -> VirtualObject {
        let mut vec = Vec::<u8>::with_capacity(h_type::get_size(data_type));
        for _ in 0..h_type::get_size(data_type){
            vec.push(u8::MAX);
        }
        VirtualObject {
            data: vec,
            data_type,
        }
    }

    pub fn set_bool(&mut self, value:&bool) {
        self.data[0] = u8::from(*value);
    }

    pub fn set_u8(&mut self, value:&u8) {
        let value = value.to_be_bytes();
        self.data.clear();
        self.data.append(&mut Vec::from(value));
    }

    pub fn set_u16(&mut self, value:&u16) {
        let value = value.to_be_bytes();
        self.data.clear();
        self.data.append(&mut Vec::from(value));
    }

    pub fn set_u32(&mut self, value:&u32) {
        let value = value.to_be_bytes();
        self.data.clear();
        self.data.append(&mut Vec::from(value));
    }

    pub fn set_u64(&mut self, value:&u64) {
        let value = value.to_be_bytes();
        self.data.clear();
        self.data.append(&mut Vec::from(value));
    }


    pub fn get_bool(&self) -> bool {
        return self.data[0] != 0;
    }

    pub fn get_u8(&self) -> u8 {
        return self.data[0];
    }

    pub fn get_u16(&self) -> u16 {
        return BigEndian::read_u16(&self.data);
    }

    pub fn get_u32(&self) -> u32 {
        return BigEndian::read_u32(&self.data);
    }

    pub fn get_u64(&self) -> u64 {
        return BigEndian::read_u64(&self.data);
    }
}

#[cfg(test)]
mod tests {
    use lib_heat_spec::h_type::HType;
    use crate::types::VirtualObject;

    #[test]
    fn virtual_object_set_get_bool() {
        let mut vobj = VirtualObject::new_empty(HType::Bool);

        vobj.set_bool(&true);

        assert_eq!(vobj.get_bool(), true);
    }


    #[test]
    fn virtual_object_set_get_u8() {
        let mut vobj = VirtualObject::new_empty(HType::U8);

        vobj.set_u8(&(u8::MAX));

        assert_eq!(vobj.get_u8(), u8::MAX);
    }

    #[test]
    fn virtual_object_set_get_u16() {
        let mut vobj = VirtualObject::new_empty(HType::U8);

        vobj.set_u16(&(u16::MAX));

        assert_eq!(vobj.get_u16(), u16::MAX);
    }

    #[test]
    fn virtual_object_set_get_u32() {
        let mut vobj = VirtualObject::new_empty(HType::U8);

        vobj.set_u32(&(u32::MAX));

        assert_eq!(vobj.get_u32(), u32::MAX);
    }

    #[test]
    fn virtual_object_set_get_u64() {
        let mut vobj = VirtualObject::new_empty(HType::U8);

        vobj.set_u64(&(u64::MAX));

        assert_eq!(vobj.get_u64(), u64::MAX);
    }

}
