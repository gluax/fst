pub trait Validate {
    fn validate(&self) -> salvo::Result<()>;
}

pub trait RoutingGroup {
    // fn group(self) ->
}
