use crate::database::db_procedures_common::{data_affect, tx_affect, Procedure};
use std::marker::PhantomData;

pub struct Proc<TData: data_affect::DataAffect, TTx: tx_affect::TxAffect> {
    pub phantom_data : PhantomData<TData>,
    pub phantom_tx : PhantomData<TTx>,
}

impl<TData, TTx> Procedure for Proc<TData, TTx> where
    TData: data_affect::TReadOnly,
    TTx: tx_affect::TNo
{
    type DataAffect = TData;
    type TxAffect = TTx;
    type InputParams = ();
    type OutputParams = ();

    fn call(&self, params: ()) -> () {
        unimplemented!()
    }
}