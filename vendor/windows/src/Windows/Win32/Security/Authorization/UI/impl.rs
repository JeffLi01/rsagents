#[cfg(feature = "Win32_Foundation")]
pub trait IEffectivePermission_Impl: Sized {
    fn GetEffectivePermission(&mut self, pguidobjecttype: *const ::windows::core::GUID, pusersid: super::super::super::Foundation::PSID, pszservername: super::super::super::Foundation::PWSTR, psd: *mut super::super::SECURITY_DESCRIPTOR, ppobjecttypelist: *mut *mut super::super::OBJECT_TYPE_LIST, pcobjecttypelistlength: *mut u32, ppgrantedaccesslist: *mut *mut u32, pcgrantedaccesslistlength: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IEffectivePermission_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEffectivePermission_Impl, const OFFSET: isize>() -> IEffectivePermission_Vtbl {
        unsafe extern "system" fn GetEffectivePermission<Identity: ::windows::core::IUnknownImpl, Impl: IEffectivePermission_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidobjecttype: *const ::windows::core::GUID, pusersid: super::super::super::Foundation::PSID, pszservername: super::super::super::Foundation::PWSTR, psd: *mut super::super::SECURITY_DESCRIPTOR, ppobjecttypelist: *mut *mut super::super::OBJECT_TYPE_LIST, pcobjecttypelistlength: *mut u32, ppgrantedaccesslist: *mut *mut u32, pcgrantedaccesslistlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetEffectivePermission(::core::mem::transmute_copy(&pguidobjecttype), ::core::mem::transmute_copy(&pusersid), ::core::mem::transmute_copy(&pszservername), ::core::mem::transmute_copy(&psd), ::core::mem::transmute_copy(&ppobjecttypelist), ::core::mem::transmute_copy(&pcobjecttypelistlength), ::core::mem::transmute_copy(&ppgrantedaccesslist), ::core::mem::transmute_copy(&pcgrantedaccesslistlength)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetEffectivePermission: GetEffectivePermission::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEffectivePermission as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEffectivePermission2_Impl: Sized {
    fn ComputeEffectivePermissionWithSecondarySecurity(
        &mut self,
        psid: super::super::super::Foundation::PSID,
        pdevicesid: super::super::super::Foundation::PSID,
        pszservername: super::super::super::Foundation::PWSTR,
        psecurityobjects: *mut SECURITY_OBJECT,
        dwsecurityobjectcount: u32,
        pusergroups: *const super::super::TOKEN_GROUPS,
        pauthzusergroupsoperations: *const super::AUTHZ_SID_OPERATION,
        pdevicegroups: *const super::super::TOKEN_GROUPS,
        pauthzdevicegroupsoperations: *const super::AUTHZ_SID_OPERATION,
        pauthzuserclaims: *const super::AUTHZ_SECURITY_ATTRIBUTES_INFORMATION,
        pauthzuserclaimsoperations: *const super::AUTHZ_SECURITY_ATTRIBUTE_OPERATION,
        pauthzdeviceclaims: *const super::AUTHZ_SECURITY_ATTRIBUTES_INFORMATION,
        pauthzdeviceclaimsoperations: *const super::AUTHZ_SECURITY_ATTRIBUTE_OPERATION,
        peffpermresultlists: *mut EFFPERM_RESULT_LIST,
    ) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IEffectivePermission2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEffectivePermission2_Impl, const OFFSET: isize>() -> IEffectivePermission2_Vtbl {
        unsafe extern "system" fn ComputeEffectivePermissionWithSecondarySecurity<Identity: ::windows::core::IUnknownImpl, Impl: IEffectivePermission2_Impl, const OFFSET: isize>(
            this: *mut ::core::ffi::c_void,
            psid: super::super::super::Foundation::PSID,
            pdevicesid: super::super::super::Foundation::PSID,
            pszservername: super::super::super::Foundation::PWSTR,
            psecurityobjects: *mut SECURITY_OBJECT,
            dwsecurityobjectcount: u32,
            pusergroups: *const super::super::TOKEN_GROUPS,
            pauthzusergroupsoperations: *const super::AUTHZ_SID_OPERATION,
            pdevicegroups: *const super::super::TOKEN_GROUPS,
            pauthzdevicegroupsoperations: *const super::AUTHZ_SID_OPERATION,
            pauthzuserclaims: *const super::AUTHZ_SECURITY_ATTRIBUTES_INFORMATION,
            pauthzuserclaimsoperations: *const super::AUTHZ_SECURITY_ATTRIBUTE_OPERATION,
            pauthzdeviceclaims: *const super::AUTHZ_SECURITY_ATTRIBUTES_INFORMATION,
            pauthzdeviceclaimsoperations: *const super::AUTHZ_SECURITY_ATTRIBUTE_OPERATION,
            peffpermresultlists: *mut EFFPERM_RESULT_LIST,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this)
                .ComputeEffectivePermissionWithSecondarySecurity(
                    ::core::mem::transmute_copy(&psid),
                    ::core::mem::transmute_copy(&pdevicesid),
                    ::core::mem::transmute_copy(&pszservername),
                    ::core::mem::transmute_copy(&psecurityobjects),
                    ::core::mem::transmute_copy(&dwsecurityobjectcount),
                    ::core::mem::transmute_copy(&pusergroups),
                    ::core::mem::transmute_copy(&pauthzusergroupsoperations),
                    ::core::mem::transmute_copy(&pdevicegroups),
                    ::core::mem::transmute_copy(&pauthzdevicegroupsoperations),
                    ::core::mem::transmute_copy(&pauthzuserclaims),
                    ::core::mem::transmute_copy(&pauthzuserclaimsoperations),
                    ::core::mem::transmute_copy(&pauthzdeviceclaims),
                    ::core::mem::transmute_copy(&pauthzdeviceclaimsoperations),
                    ::core::mem::transmute_copy(&peffpermresultlists),
                )
                .into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ComputeEffectivePermissionWithSecondarySecurity: ComputeEffectivePermissionWithSecondarySecurity::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEffectivePermission2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
pub trait ISecurityInformation_Impl: Sized {
    fn GetObjectInformation(&mut self, pobjectinfo: *mut SI_OBJECT_INFO) -> ::windows::core::Result<()>;
    fn GetSecurity(&mut self, requestedinformation: super::super::OBJECT_SECURITY_INFORMATION, ppsecuritydescriptor: *mut *mut super::super::SECURITY_DESCRIPTOR, fdefault: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetSecurity(&mut self, securityinformation: super::super::OBJECT_SECURITY_INFORMATION, psecuritydescriptor: *mut super::super::SECURITY_DESCRIPTOR) -> ::windows::core::Result<()>;
    fn GetAccessRights(&mut self, pguidobjecttype: *const ::windows::core::GUID, dwflags: SECURITY_INFO_PAGE_FLAGS, ppaccess: *mut *mut SI_ACCESS, pcaccesses: *mut u32, pidefaultaccess: *mut u32) -> ::windows::core::Result<()>;
    fn MapGeneric(&mut self, pguidobjecttype: *const ::windows::core::GUID, paceflags: *mut u8, pmask: *mut u32) -> ::windows::core::Result<()>;
    fn GetInheritTypes(&mut self, ppinherittypes: *mut *mut SI_INHERIT_TYPE, pcinherittypes: *mut u32) -> ::windows::core::Result<()>;
    fn PropertySheetPageCallback(&mut self, hwnd: super::super::super::Foundation::HWND, umsg: super::super::super::UI::Controls::PSPCB_MESSAGE, upage: SI_PAGE_TYPE) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ISecurityInformation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecurityInformation_Impl, const OFFSET: isize>() -> ISecurityInformation_Vtbl {
        unsafe extern "system" fn GetObjectInformation<Identity: ::windows::core::IUnknownImpl, Impl: ISecurityInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobjectinfo: *mut SI_OBJECT_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetObjectInformation(::core::mem::transmute_copy(&pobjectinfo)).into()
        }
        unsafe extern "system" fn GetSecurity<Identity: ::windows::core::IUnknownImpl, Impl: ISecurityInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestedinformation: super::super::OBJECT_SECURITY_INFORMATION, ppsecuritydescriptor: *mut *mut super::super::SECURITY_DESCRIPTOR, fdefault: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSecurity(::core::mem::transmute_copy(&requestedinformation), ::core::mem::transmute_copy(&ppsecuritydescriptor), ::core::mem::transmute_copy(&fdefault)).into()
        }
        unsafe extern "system" fn SetSecurity<Identity: ::windows::core::IUnknownImpl, Impl: ISecurityInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, securityinformation: super::super::OBJECT_SECURITY_INFORMATION, psecuritydescriptor: *mut super::super::SECURITY_DESCRIPTOR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSecurity(::core::mem::transmute_copy(&securityinformation), ::core::mem::transmute_copy(&psecuritydescriptor)).into()
        }
        unsafe extern "system" fn GetAccessRights<Identity: ::windows::core::IUnknownImpl, Impl: ISecurityInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidobjecttype: *const ::windows::core::GUID, dwflags: SECURITY_INFO_PAGE_FLAGS, ppaccess: *mut *mut SI_ACCESS, pcaccesses: *mut u32, pidefaultaccess: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetAccessRights(::core::mem::transmute_copy(&pguidobjecttype), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&ppaccess), ::core::mem::transmute_copy(&pcaccesses), ::core::mem::transmute_copy(&pidefaultaccess)).into()
        }
        unsafe extern "system" fn MapGeneric<Identity: ::windows::core::IUnknownImpl, Impl: ISecurityInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidobjecttype: *const ::windows::core::GUID, paceflags: *mut u8, pmask: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MapGeneric(::core::mem::transmute_copy(&pguidobjecttype), ::core::mem::transmute_copy(&paceflags), ::core::mem::transmute_copy(&pmask)).into()
        }
        unsafe extern "system" fn GetInheritTypes<Identity: ::windows::core::IUnknownImpl, Impl: ISecurityInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppinherittypes: *mut *mut SI_INHERIT_TYPE, pcinherittypes: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetInheritTypes(::core::mem::transmute_copy(&ppinherittypes), ::core::mem::transmute_copy(&pcinherittypes)).into()
        }
        unsafe extern "system" fn PropertySheetPageCallback<Identity: ::windows::core::IUnknownImpl, Impl: ISecurityInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, umsg: super::super::super::UI::Controls::PSPCB_MESSAGE, upage: SI_PAGE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PropertySheetPageCallback(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&umsg), ::core::mem::transmute_copy(&upage)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetObjectInformation: GetObjectInformation::<Identity, Impl, OFFSET>,
            GetSecurity: GetSecurity::<Identity, Impl, OFFSET>,
            SetSecurity: SetSecurity::<Identity, Impl, OFFSET>,
            GetAccessRights: GetAccessRights::<Identity, Impl, OFFSET>,
            MapGeneric: MapGeneric::<Identity, Impl, OFFSET>,
            GetInheritTypes: GetInheritTypes::<Identity, Impl, OFFSET>,
            PropertySheetPageCallback: PropertySheetPageCallback::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISecurityInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ISecurityInformation2_Impl: Sized {
    fn IsDaclCanonical(&mut self, pdacl: *mut super::super::ACL) -> super::super::super::Foundation::BOOL;
    fn LookupSids(&mut self, csids: u32, rgpsids: *mut super::super::super::Foundation::PSID, ppdo: *mut ::core::option::Option<super::super::super::System::Com::IDataObject>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ISecurityInformation2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecurityInformation2_Impl, const OFFSET: isize>() -> ISecurityInformation2_Vtbl {
        unsafe extern "system" fn IsDaclCanonical<Identity: ::windows::core::IUnknownImpl, Impl: ISecurityInformation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdacl: *mut super::super::ACL) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsDaclCanonical(::core::mem::transmute_copy(&pdacl))
        }
        unsafe extern "system" fn LookupSids<Identity: ::windows::core::IUnknownImpl, Impl: ISecurityInformation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, csids: u32, rgpsids: *mut super::super::super::Foundation::PSID, ppdo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LookupSids(::core::mem::transmute_copy(&csids), ::core::mem::transmute_copy(&rgpsids), ::core::mem::transmute_copy(&ppdo)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            IsDaclCanonical: IsDaclCanonical::<Identity, Impl, OFFSET>,
            LookupSids: LookupSids::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISecurityInformation2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISecurityInformation3_Impl: Sized {
    fn GetFullResourceName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn OpenElevatedEditor(&mut self, hwnd: super::super::super::Foundation::HWND, upage: SI_PAGE_TYPE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISecurityInformation3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecurityInformation3_Impl, const OFFSET: isize>() -> ISecurityInformation3_Vtbl {
        unsafe extern "system" fn GetFullResourceName<Identity: ::windows::core::IUnknownImpl, Impl: ISecurityInformation3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszresourcename: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFullResourceName() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszresourcename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenElevatedEditor<Identity: ::windows::core::IUnknownImpl, Impl: ISecurityInformation3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, upage: SI_PAGE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OpenElevatedEditor(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&upage)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetFullResourceName: GetFullResourceName::<Identity, Impl, OFFSET>,
            OpenElevatedEditor: OpenElevatedEditor::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISecurityInformation3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISecurityInformation4_Impl: Sized {
    fn GetSecondarySecurity(&mut self, psecurityobjects: *mut *mut SECURITY_OBJECT, psecurityobjectcount: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISecurityInformation4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecurityInformation4_Impl, const OFFSET: isize>() -> ISecurityInformation4_Vtbl {
        unsafe extern "system" fn GetSecondarySecurity<Identity: ::windows::core::IUnknownImpl, Impl: ISecurityInformation4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psecurityobjects: *mut *mut SECURITY_OBJECT, psecurityobjectcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSecondarySecurity(::core::mem::transmute_copy(&psecurityobjects), ::core::mem::transmute_copy(&psecurityobjectcount)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetSecondarySecurity: GetSecondarySecurity::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISecurityInformation4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISecurityObjectTypeInfo_Impl: Sized {
    fn GetInheritSource(&mut self, si: u32, pacl: *mut super::super::ACL, ppinheritarray: *mut *mut super::INHERITED_FROMA) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISecurityObjectTypeInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecurityObjectTypeInfo_Impl, const OFFSET: isize>() -> ISecurityObjectTypeInfo_Vtbl {
        unsafe extern "system" fn GetInheritSource<Identity: ::windows::core::IUnknownImpl, Impl: ISecurityObjectTypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, si: u32, pacl: *mut super::super::ACL, ppinheritarray: *mut *mut super::INHERITED_FROMA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetInheritSource(::core::mem::transmute_copy(&si), ::core::mem::transmute_copy(&pacl), ::core::mem::transmute_copy(&ppinheritarray)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetInheritSource: GetInheritSource::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISecurityObjectTypeInfo as ::windows::core::Interface>::IID
    }
}
