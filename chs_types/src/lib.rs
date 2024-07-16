#[derive(Default)]
pub struct TBuider {
    value: Typ,
}

impl From<Typ> for TBuider {
    fn from(value: Typ) -> Self {
        Self { value }
    }
}

impl TBuider {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn char_t(mut self) -> Self {
        self.value = Typ::Primitive(Primitive::Char);
        self
    }
    pub fn void(mut self) -> Self {
        self.value = Typ::Primitive(Primitive::Void);
        self
    }
    pub fn int(mut self) -> Self {
        self.value = Typ::Primitive(Primitive::Int);
        self
    }
    pub fn struct_t(mut self) -> Self {
        self.value = Typ::UserDefined(UserDefined::Struct(Vec::default()));
        self
    }
    pub fn field(mut self, field: Typ) -> Self {
        if let Typ::UserDefined(UserDefined::Struct(ref mut s)) = self.value {
            s.push(field);
        }
        self
    }
    pub fn ptr_to(mut self) -> Self {
        let tmp = self.value;
        self.value = Typ::Pointer(Pointer::PtrTo(to_mask(tmp)));
        self
    }
    pub fn build(self) -> Typ {
        self.value
    }
}

type Mask<T> = Box<T>;

fn to_mask<T>(tmp: T) -> Mask<T> {
    Box::new(tmp)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Typ {
    Primitive(Primitive),
    Pointer(Pointer),
    UserDefined(UserDefined),
}

impl Default for Typ {
    fn default() -> Self {
        Self::Primitive(Primitive::Void)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserDefined {
    Struct(Vec<Typ>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pointer {
    PtrTo(Mask<Typ>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Primitive {
    Int,
    Char,
    Void,
}
