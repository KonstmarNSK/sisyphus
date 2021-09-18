use std::marker::PhantomData;

/// Description of possible procedure's impact on data in database
pub mod data_affect {
    pub trait DataAffect {}

    /// Procedure typed by ReadOnly can NOT change any data
    pub trait ReadOnly : DataAffect{}

    /// Procedure typed by MayWrite CAN change some data
    pub trait MayWrite : DataAffect{}
}

/// Description of possible procedure's transaction usage (impact on current tx, create new tx etc.)
pub mod tx_affect {
    pub trait TxAffect {}

    /// Procedure typed by No doesn't create any transaction and doesn't affect current transaction
    pub trait No : TxAffect{}
}


pub trait Procedure {
    type DataAffect: data_affect::DataAffect;
    type TxAffect: tx_affect::TxAffect;

    type InputParams;
    type OutputParams;


    fn call(&self, params: Self::InputParams) -> Self::OutputParams;
}



pub struct Proc<TData: data_affect::DataAffect, TTx: tx_affect::TxAffect> {
    phantom_data : PhantomData<TData>,
    phantom_tx : PhantomData<TTx>,
}

impl<TData, TTx> Procedure for Proc<TData, TTx> where
    TData: data_affect::DataAffect,
    TTx: tx_affect::TxAffect
{
    type DataAffect = TData;
    type TxAffect = TTx;
    type InputParams = ();
    type OutputParams = ();

    fn call(&self, params: ()) -> () {
        unimplemented!()
    }
}