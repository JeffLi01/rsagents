pub trait ICompositionInteractionSource_Impl: Sized {}
impl ::windows::core::RuntimeName for ICompositionInteractionSource {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.ICompositionInteractionSource";
}
impl ICompositionInteractionSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionInteractionSource_Impl, const OFFSET: isize>() -> ICompositionInteractionSource_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionInteractionSource, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionInteractionSource as ::windows::core::Interface>::IID
    }
}
pub trait IInteractionTrackerOwner_Impl: Sized {
    fn CustomAnimationStateEntered(&mut self, sender: &::core::option::Option<InteractionTracker>, args: &::core::option::Option<InteractionTrackerCustomAnimationStateEnteredArgs>) -> ::windows::core::Result<()>;
    fn IdleStateEntered(&mut self, sender: &::core::option::Option<InteractionTracker>, args: &::core::option::Option<InteractionTrackerIdleStateEnteredArgs>) -> ::windows::core::Result<()>;
    fn InertiaStateEntered(&mut self, sender: &::core::option::Option<InteractionTracker>, args: &::core::option::Option<InteractionTrackerInertiaStateEnteredArgs>) -> ::windows::core::Result<()>;
    fn InteractingStateEntered(&mut self, sender: &::core::option::Option<InteractionTracker>, args: &::core::option::Option<InteractionTrackerInteractingStateEnteredArgs>) -> ::windows::core::Result<()>;
    fn RequestIgnored(&mut self, sender: &::core::option::Option<InteractionTracker>, args: &::core::option::Option<InteractionTrackerRequestIgnoredArgs>) -> ::windows::core::Result<()>;
    fn ValuesChanged(&mut self, sender: &::core::option::Option<InteractionTracker>, args: &::core::option::Option<InteractionTrackerValuesChangedArgs>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IInteractionTrackerOwner {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerOwner";
}
impl IInteractionTrackerOwner_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerOwner_Impl, const OFFSET: isize>() -> IInteractionTrackerOwner_Vtbl {
        unsafe extern "system" fn CustomAnimationStateEntered<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerOwner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CustomAnimationStateEntered(::core::mem::transmute(&sender), ::core::mem::transmute(&args)).into()
        }
        unsafe extern "system" fn IdleStateEntered<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerOwner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IdleStateEntered(::core::mem::transmute(&sender), ::core::mem::transmute(&args)).into()
        }
        unsafe extern "system" fn InertiaStateEntered<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerOwner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InertiaStateEntered(::core::mem::transmute(&sender), ::core::mem::transmute(&args)).into()
        }
        unsafe extern "system" fn InteractingStateEntered<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerOwner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InteractingStateEntered(::core::mem::transmute(&sender), ::core::mem::transmute(&args)).into()
        }
        unsafe extern "system" fn RequestIgnored<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerOwner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RequestIgnored(::core::mem::transmute(&sender), ::core::mem::transmute(&args)).into()
        }
        unsafe extern "system" fn ValuesChanged<Identity: ::windows::core::IUnknownImpl, Impl: IInteractionTrackerOwner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ValuesChanged(::core::mem::transmute(&sender), ::core::mem::transmute(&args)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInteractionTrackerOwner, OFFSET>(),
            CustomAnimationStateEntered: CustomAnimationStateEntered::<Identity, Impl, OFFSET>,
            IdleStateEntered: IdleStateEntered::<Identity, Impl, OFFSET>,
            InertiaStateEntered: InertiaStateEntered::<Identity, Impl, OFFSET>,
            InteractingStateEntered: InteractingStateEntered::<Identity, Impl, OFFSET>,
            RequestIgnored: RequestIgnored::<Identity, Impl, OFFSET>,
            ValuesChanged: ValuesChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInteractionTrackerOwner as ::windows::core::Interface>::IID
    }
}
