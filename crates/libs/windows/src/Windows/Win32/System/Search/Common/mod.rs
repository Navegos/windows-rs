#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CONDITION_OPERATION(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CONDITION_TYPE(pub i32);
pub const COP_APPLICATION_SPECIFIC: CONDITION_OPERATION = CONDITION_OPERATION(14i32);
pub const COP_DOSWILDCARDS: CONDITION_OPERATION = CONDITION_OPERATION(11i32);
pub const COP_EQUAL: CONDITION_OPERATION = CONDITION_OPERATION(1i32);
pub const COP_GREATERTHAN: CONDITION_OPERATION = CONDITION_OPERATION(4i32);
pub const COP_GREATERTHANOREQUAL: CONDITION_OPERATION = CONDITION_OPERATION(6i32);
pub const COP_IMPLICIT: CONDITION_OPERATION = CONDITION_OPERATION(0i32);
pub const COP_LESSTHAN: CONDITION_OPERATION = CONDITION_OPERATION(3i32);
pub const COP_LESSTHANOREQUAL: CONDITION_OPERATION = CONDITION_OPERATION(5i32);
pub const COP_NOTEQUAL: CONDITION_OPERATION = CONDITION_OPERATION(2i32);
pub const COP_VALUE_CONTAINS: CONDITION_OPERATION = CONDITION_OPERATION(9i32);
pub const COP_VALUE_ENDSWITH: CONDITION_OPERATION = CONDITION_OPERATION(8i32);
pub const COP_VALUE_NOTCONTAINS: CONDITION_OPERATION = CONDITION_OPERATION(10i32);
pub const COP_VALUE_STARTSWITH: CONDITION_OPERATION = CONDITION_OPERATION(7i32);
pub const COP_WORD_EQUAL: CONDITION_OPERATION = CONDITION_OPERATION(12i32);
pub const COP_WORD_STARTSWITH: CONDITION_OPERATION = CONDITION_OPERATION(13i32);
pub const CT_AND_CONDITION: CONDITION_TYPE = CONDITION_TYPE(0i32);
pub const CT_LEAF_CONDITION: CONDITION_TYPE = CONDITION_TYPE(3i32);
pub const CT_NOT_CONDITION: CONDITION_TYPE = CONDITION_TYPE(2i32);
pub const CT_OR_CONDITION: CONDITION_TYPE = CONDITION_TYPE(1i32);
