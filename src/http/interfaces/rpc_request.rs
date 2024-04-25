pub trait RpcRequest {
    fn length() -> usize;

    fn is_empty() -> bool{
        false
    }

    fn validate(&self);
}