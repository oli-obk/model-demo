use model_demo::state_machine;

state_machine!{
    Phone,
    begin_incoming_call: HangedUp -> Ringing,
    accept_call: Ringing -> Calling,
    end_call: Calling -> HangedUp,
    begin_outgoing_call: HangedUp -> Dialing,
    finished_dialing: Dialing -> Waiting,
    call_accepted: Waiting -> Calling,
}

#[test]
fn succeed() {
    let mut phone = Phone::HangedUp;
    phone.begin_incoming_call();
    phone.accept_call();
    phone.end_call();
    phone.begin_outgoing_call();
    phone.finished_dialing();
    phone.call_accepted();
    phone.end_call();
}


#[test]
#[should_panic]
fn fail() {
    let mut phone = Phone::HangedUp;
    phone.accept_call();
}
