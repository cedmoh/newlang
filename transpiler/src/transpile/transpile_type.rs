use core::ast::Type;
use crustal as C;

pub fn transpile_type(ty: Type) -> C::Type {
    match ty.id.as_str() {
        "int" => C::Type::new_int32(),
        "float" => C::Type::new_float(),
        "bool" => C::Type::new_bool(),
        _ => todo!(),
    }
}
