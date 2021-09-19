use std::marker::PhantomData;

/// Description of possible procedure's impact on data in database
pub mod data_affect {
    pub trait DataAffect {}

    /// Procedure typed by ReadOnly can NOT change any data
    pub struct ReadOnly;
    pub trait TReadOnly : DataAffect{}
    impl DataAffect for ReadOnly{}
    impl TReadOnly for ReadOnly{}

    /// Procedure typed by MayWrite CAN change some data
    pub struct  MayWrite;
    pub trait TMayWrite : DataAffect{}
    impl DataAffect for MayWrite{}
    impl TMayWrite for MayWrite{}
}

/// Description of possible procedure's transaction usage (impact on current tx, create new tx etc.)
pub mod tx_affect {
    pub trait TxAffect {}

    /// Procedure typed by No doesn't create any transaction and doesn't affect current transaction
    pub struct No;
    pub trait TNo : TxAffect{}
    impl TxAffect for No{}
    impl TNo for No{}
}


pub trait Procedure {
    type DataAffect: data_affect::DataAffect;
    type TxAffect: tx_affect::TxAffect;

    type InputParams;
    type OutputParams;


    fn call(&self, params: Self::InputParams) -> Self::OutputParams;
}