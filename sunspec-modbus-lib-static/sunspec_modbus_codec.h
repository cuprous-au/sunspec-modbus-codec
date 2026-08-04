#ifndef SUNSPEC_MODBUS_CODEC_H
#define SUNSPEC_MODBUS_CODEC_H

#pragma once

/* Generated with cbindgen:0.29.4 */

#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

enum St
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  St_Off = 1,
  St_On = 2,
  St_Full = 3,
  St_Fault = 4,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum St St;
#else
typedef uint16_t St;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum Ctl
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  Ctl_None = 0,
  Ctl_Automatic = 1,
  Ctl_ForceOff = 2,
  Ctl_Test = 3,
  Ctl_Throttle = 4,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum Ctl Ctl;
#else
typedef uint16_t Ctl;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum Alg
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   For test purposes only
   */
  Alg_None = 0,
  Alg_AesGmac64 = 1,
  Alg_Ecc256 = 2,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum Alg Alg;
#else
typedef uint16_t Alg;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum Sts
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  Sts_Success = 0,
  Sts_Ds = 1,
  /*
   One or more registers were not writable by this role
   */
  Sts_Acl = 2,
  /*
   Offset out of range or missing from multi-register value
   */
  Sts_Off = 3,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum Sts Sts;
#else
typedef uint16_t Sts;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum Alm
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  Alm_None = 0,
  /*
   Tampered
   */
  Alm_Alm = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum Alm Alm;
#else
typedef uint16_t Alm;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum Fmt
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  Fmt_None = 0,
  Fmt_X509Pem = 1,
  Fmt_X509Der = 2,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum Fmt Fmt;
#else
typedef uint16_t Fmt;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum Typ
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  Typ_Unknown = 0,
  Typ_Internal = 1,
  Typ_TwistedPair = 2,
  Typ_Fiber = 3,
  Typ_Wireless = 4,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum Typ Typ;
#else
typedef uint16_t Typ;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum CfgSt
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  CfgSt_NotConfigured = 0,
  CfgSt_ValidSetting = 1,
  CfgSt_ValidHw = 2,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum CfgSt CfgSt;
#else
typedef uint16_t CfgSt;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum Cfg
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  Cfg_Static = 0,
  Cfg_Dhcp = 1,
  Cfg_Bootp = 2,
  Cfg_Zeroconf = 3,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum Cfg Cfg;
#else
typedef uint16_t Cfg;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum Pty
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  Pty_None = 0,
  Pty_Odd = 1,
  Pty_Even = 2,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum Pty Pty;
#else
typedef uint16_t Pty;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum Dup
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  Dup_Full = 0,
  Dup_Half = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum Dup Dup;
#else
typedef uint16_t Dup;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum Flw
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  Flw_None = 0,
  Flw_Hw = 1,
  Flw_Xonxoff = 2,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum Flw Flw;
#else
typedef uint16_t Flw;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum Pcol
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  Pcol_Unknown = 0,
  Pcol_Modbus = 1,
  Pcol_Vendor = 2,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum Pcol Pcol;
#else
typedef uint16_t Pcol;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum Auth
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  Auth_None = 0,
  Auth_Pap = 1,
  Auth_Chap = 2,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum Auth Auth;
#else
typedef uint16_t Auth;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum DerTyp
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  DerTyp_Pv = 4,
  DerTyp_PvStor = 82,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum DerTyp DerTyp;
#else
typedef uint16_t DerTyp;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum VArAct
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  VArAct_Switch = 1,
  VArAct_Maintain = 2,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum VArAct VArAct;
#else
typedef uint16_t VArAct;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum ClcTotVa
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  ClcTotVa_Vector = 1,
  ClcTotVa_Arithmetic = 2,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum ClcTotVa ClcTotVa;
#else
typedef uint16_t ClcTotVa;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum ConnPh
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  ConnPh_A = 1,
  ConnPh_B = 2,
  ConnPh_C = 3,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum ConnPh ConnPh;
#else
typedef uint16_t ConnPh;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum Conn
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  Conn_Disconnect = 0,
  Conn_Connect = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum Conn Conn;
#else
typedef uint16_t Conn;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum WMaxLimEna
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  WMaxLimEna_Disabled = 0,
  WMaxLimEna_Enabled = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum WMaxLimEna WMaxLimEna;
#else
typedef uint16_t WMaxLimEna;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum OutPfSetEna
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  OutPfSetEna_Disabled = 0,
  OutPfSetEna_Enabled = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum OutPfSetEna OutPfSetEna;
#else
typedef uint16_t OutPfSetEna;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum VArPctMod
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  VArPctMod_None = 0,
  VArPctMod_WMax = 1,
  VArPctMod_VArMax = 2,
  VArPctMod_VArAval = 3,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum VArPctMod VArPctMod;
#else
typedef uint16_t VArPctMod;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum VArPctEna
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  VArPctEna_Disabled = 0,
  VArPctEna_Enabled = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum VArPctEna VArPctEna;
#else
typedef uint16_t VArPctEna;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum ChaSt
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  ChaSt_Off = 1,
  ChaSt_Empty = 2,
  ChaSt_Discharging = 3,
  ChaSt_Charging = 4,
  ChaSt_Full = 5,
  ChaSt_Holding = 6,
  ChaSt_Testing = 7,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum ChaSt ChaSt;
#else
typedef uint16_t ChaSt;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum ChaGriSet
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  ChaGriSet_Pv = 0,
  ChaGriSet_Grid = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum ChaGriSet ChaGriSet;
#else
typedef uint16_t ChaGriSet;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum SigType
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  SigType_Unknown = 0,
  SigType_Absolute = 1,
  SigType_Relative = 2,
  SigType_Multiplier = 3,
  SigType_Level = 4,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum SigType SigType;
#else
typedef uint16_t SigType;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum ArGraMod
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  ArGraMod_Edge = 0,
  ArGraMod_Center = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum ArGraMod ArGraMod;
#else
typedef uint16_t ArGraMod;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum CrvType
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  CrvType_CeaseToEnergize = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum CrvType CrvType;
#else
typedef uint16_t CrvType;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum Stat
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  Stat_Off = 1,
  Stat_Sleeping = 2,
  Stat_Starting = 3,
  Stat_Mppt = 4,
  Stat_Throttled = 5,
  Stat_ShuttingDown = 6,
  Stat_Fault = 7,
  Stat_Standby = 8,
  Stat_Test = 9,
  Stat_Other = 10,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum Stat Stat;
#else
typedef uint16_t Stat;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum AcType
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Single Phase
   */
  AcType_SinglePhase = 0,
  /*
   Split Phase
   */
  AcType_SplitPhase = 1,
  /*
   Three Phase
   */
  AcType_ThreePhase = 2,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum AcType AcType;
#else
typedef uint16_t AcType;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum InvSt
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  InvSt_Off = 0,
  InvSt_Sleeping = 1,
  InvSt_Starting = 2,
  InvSt_Running = 3,
  InvSt_Throttled = 4,
  InvSt_ShuttingDown = 5,
  InvSt_Fault = 6,
  InvSt_Standby = 7,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum InvSt InvSt;
#else
typedef uint16_t InvSt;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum ConnSt
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Disconnected
   
   Disconnected from the grid.
   */
  ConnSt_Disconnected = 0,
  /*
   Connected
   
   Connected to the grid.
   */
  ConnSt_Connected = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum ConnSt ConnSt;
#else
typedef uint16_t ConnSt;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum Es
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  Es_Disabled = 0,
  Es_Enabled = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum Es Es;
#else
typedef uint16_t Es;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum PfwInjEna
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Disabled
   
   Function is disabled.
   */
  PfwInjEna_Disabled = 0,
  /*
   Enabled
   
   Function is enabled.
   */
  PfwInjEna_Enabled = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum PfwInjEna PfwInjEna;
#else
typedef uint16_t PfwInjEna;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum PfwInjEnaRvrt
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Disabled
   
   Function is disabled.
   */
  PfwInjEnaRvrt_Disabled = 0,
  /*
   Enabled
   
   Function is enabled.
   */
  PfwInjEnaRvrt_Enabled = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum PfwInjEnaRvrt PfwInjEnaRvrt;
#else
typedef uint16_t PfwInjEnaRvrt;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum PfwAbsEna
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Disabled
   
   Function is disabled.
   */
  PfwAbsEna_Disabled = 0,
  /*
   Enabled
   
   Function is enabled.
   */
  PfwAbsEna_Enabled = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum PfwAbsEna PfwAbsEna;
#else
typedef uint16_t PfwAbsEna;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum PfwAbsEnaRvrt
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Disabled
   
   Function is disabled.
   */
  PfwAbsEnaRvrt_Disabled = 0,
  /*
   Enabled
   
   Function is enabled.
   */
  PfwAbsEnaRvrt_Enabled = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum PfwAbsEnaRvrt PfwAbsEnaRvrt;
#else
typedef uint16_t PfwAbsEnaRvrt;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum WMaxLimPctEna
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Disabled
   
   Function is disabled.
   */
  WMaxLimPctEna_Disabled = 0,
  /*
   Enabled
   
   Function is enabled.
   */
  WMaxLimPctEna_Enabled = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum WMaxLimPctEna WMaxLimPctEna;
#else
typedef uint16_t WMaxLimPctEna;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum WMaxLimPctEnaRvrt
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Disabled
   
   Function is disabled.
   */
  WMaxLimPctEnaRvrt_Disabled = 0,
  /*
   Enabled
   
   Function is enabled.
   */
  WMaxLimPctEnaRvrt_Enabled = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum WMaxLimPctEnaRvrt WMaxLimPctEnaRvrt;
#else
typedef uint16_t WMaxLimPctEnaRvrt;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum WSetEna
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Disabled
   
   Function is disabled.
   */
  WSetEna_Disabled = 0,
  /*
   Enabled
   
   Function is enabled.
   */
  WSetEna_Enabled = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum WSetEna WSetEna;
#else
typedef uint16_t WSetEna;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum WSetMod
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Active Power As Max Percent
   
   Active power setting is percentage of maximum active power.
   */
  WSetMod_WMaxPct = 0,
  /*
   Active Power As Watts
   
   Active power setting is in watts.
   */
  WSetMod_Watts = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum WSetMod WSetMod;
#else
typedef uint16_t WSetMod;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum WSetEnaRvrt
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Disabled
   
   Function is disabled.
   */
  WSetEnaRvrt_Disabled = 0,
  /*
   Enabled
   
   Function is enabled.
   */
  WSetEnaRvrt_Enabled = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum WSetEnaRvrt WSetEnaRvrt;
#else
typedef uint16_t WSetEnaRvrt;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum VarSetEna
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Disabled
   
   Function is disabled.
   */
  VarSetEna_Disabled = 0,
  /*
   Enabled
   
   Function is enabled.
   */
  VarSetEna_Enabled = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum VarSetEna VarSetEna;
#else
typedef uint16_t VarSetEna;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum VarSetMod
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Reactive Power As Watt Max Pct
   
   Reactive power setting is percent of maximum active power.
   */
  VarSetMod_WMaxPct = 0,
  /*
   Reactive Power As Var Max Pct
   
   Reactive power setting is percent of maximum reactive power.
   */
  VarSetMod_VarMaxPct = 1,
  /*
   Reactive Power As Var Avail Pct
   
   Reactive power setting is percent of available reactive  power.
   */
  VarSetMod_VarAvailPct = 2,
  /*
   Reactive Power As VA Max Pct
   
   Reactive power setting is percent of maximum apparent power.
   */
  VarSetMod_VaMaxPct = 3,
  /*
   Reactive Power As Vars
   
   Reactive power is in vars.
   */
  VarSetMod_Vars = 4,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum VarSetMod VarSetMod;
#else
typedef uint16_t VarSetMod;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum VarSetPri
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Active Power Priority
   
   Active power priority.
   */
  VarSetPri_Active = 0,
  /*
   Reactive Power Priority
   
   Reactive power priority.
   */
  VarSetPri_Reactive = 1,
  /*
   Vendor Power Priority
   
   Power priority is vendor specific mode.
   */
  VarSetPri_Vendor = 2,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum VarSetPri VarSetPri;
#else
typedef uint16_t VarSetPri;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum VarSetEnaRvrt
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Disabled
   
   Function is disabled.
   */
  VarSetEnaRvrt_Disabled = 0,
  /*
   Enabled
   
   Function is enabled.
   */
  VarSetEnaRvrt_Enabled = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum VarSetEnaRvrt VarSetEnaRvrt;
#else
typedef uint16_t VarSetEnaRvrt;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum WRmpRef
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Max Current Ramp
   
   Ramp based on percent of max current per second.
   */
  WRmpRef_AMax = 0,
  /*
   Max Active Power Ramp
   
   Ramp based on percent of max active power per second.
   */
  WRmpRef_WMax = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum WRmpRef WRmpRef;
#else
typedef uint16_t WRmpRef;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum AntiIslEna
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Disabled
   
   Anti-islanding is disabled.
   */
  AntiIslEna_Disabled = 0,
  /*
   Enabled
   
   Anti-islanding is enabled.
   */
  AntiIslEna_Enabled = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum AntiIslEna AntiIslEna;
#else
typedef uint16_t AntiIslEna;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum Ena
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Disabled
   
   Function is disabled.
   */
  Ena_Disabled = 0,
  /*
   Enabled
   
   Function is enabled.
   */
  Ena_Enabled = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum Ena Ena;
#else
typedef uint16_t Ena;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum AdptCrvRslt
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Update In Progress
   
   Curve update in progress.
   */
  AdptCrvRslt_InProgress = 0,
  /*
   Update Complete
   
   Curve update completed successfully.
   */
  AdptCrvRslt_Completed = 1,
  /*
   Update Failed
   
   Curve update failed.
   */
  AdptCrvRslt_Failed = 2,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum AdptCrvRslt AdptCrvRslt;
#else
typedef uint16_t AdptCrvRslt;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum AdptCtlRslt
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Update In Progress
   
   Control update in progress.
   */
  AdptCtlRslt_InProgress = 0,
  /*
   Update Complete
   
   Control update completed successfully.
   */
  AdptCtlRslt_Completed = 1,
  /*
   Update Failed
   
   Control update failed.
   */
  AdptCtlRslt_Failed = 2,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum AdptCtlRslt AdptCtlRslt;
#else
typedef uint16_t AdptCtlRslt;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum Sta
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   OK
   
   No warnings or errors pending.
   */
  Sta_Ok = 0,
  /*
   Warning
   
   One or more warnings pending.
   */
  Sta_Warning = 1,
  /*
   Error
   
   One or more errors pending.
   */
  Sta_Error = 2,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum Sta Sta;
#else
typedef uint16_t Sta;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum LocRemCtl
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Remote Control
   */
  LocRemCtl_Remote = 0,
  /*
   Local Control
   
   Local mode is required for manual/maintenance operations. Once invoked, it must be explicitly exited for the inverter to be controlled remotely.
   */
  LocRemCtl_Local = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum LocRemCtl LocRemCtl;
#else
typedef uint16_t LocRemCtl;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum OpCtl
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Stop the DER
   */
  OpCtl_Stop = 0,
  /*
   Start the DER
   */
  OpCtl_Start = 1,
  /*
   Enter Standby Mode
   */
  OpCtl_EnterStandby = 2,
  /*
   Exit Standby Mode
   */
  OpCtl_ExitStandby = 3,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum OpCtl OpCtl;
#else
typedef uint16_t OpCtl;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum State
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  State_Disconnected = 1,
  State_Initializing = 2,
  State_Connected = 3,
  State_Standby = 4,
  State_SocProtection = 5,
  State_Suspending = 6,
  State_Fault = 99,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum State State;
#else
typedef uint16_t State;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum ReqInvState
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  ReqInvState_NoRequest = 0,
  /*
   Battery is notified of inverter state change through SetInvState.
   */
  ReqInvState_Start = 1,
  /*
   Battery is notified of inverter state change through SetInvState.
   */
  ReqInvState_Stop = 2,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum ReqInvState ReqInvState;
#else
typedef uint16_t ReqInvState;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum SetOp
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  SetOp_Connect = 1,
  SetOp_Disconnect = 2,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum SetOp SetOp;
#else
typedef uint16_t SetOp;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum SetInvState
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  SetInvState_InverterStopped = 1,
  SetInvState_InverterStandby = 2,
  SetInvState_InverterStarted = 3,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum SetInvState SetInvState;
#else
typedef uint16_t SetInvState;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum ConFail
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  ConFail_NoFailure = 0,
  ConFail_ButtonPushed = 1,
  ConFail_StrGroundFault = 2,
  ConFail_OutsideVoltageRange = 3,
  ConFail_StringNotEnabled = 4,
  ConFail_FuseOpen = 5,
  ConFail_ContactorFailure = 6,
  ConFail_PrechargeFailure = 7,
  /*
   See Evt1 for more information.
   */
  ConFail_StringFault = 8,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum ConFail ConFail;
#else
typedef uint16_t ConFail;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum SetCon
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  SetCon_ConnectString = 1,
  SetCon_DisconnectString = 2,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum SetCon SetCon;
#else
typedef uint16_t SetCon;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum ChargerSt
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  ChargerSt_Off = 0,
  ChargerSt_Float = 1,
  ChargerSt_Bulk = 2,
  ChargerSt_Absorb = 3,
  ChargerSt_Eq = 4,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum ChargerSt ChargerSt;
#else
typedef uint16_t ChargerSt;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum CcConfigMpptMode
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  CcConfigMpptMode_Auto = 0,
  CcConfigMpptMode_UPick = 1,
  CcConfigMpptMode_Wind = 2,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum CcConfigMpptMode CcConfigMpptMode;
#else
typedef uint16_t CcConfigMpptMode;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum CcConfigSweepWidth
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  CcConfigSweepWidth_Half = 0,
  CcConfigSweepWidth_Full = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum CcConfigSweepWidth CcConfigSweepWidth;
#else
typedef uint16_t CcConfigSweepWidth;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum CcConfigSweepMax
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  CcConfigSweepMax_EightyPercent = 0,
  CcConfigSweepMax_EightyFivePercent = 1,
  CcConfigSweepMax_NintyPercent = 2,
  CcConfigSweepMax_NintyNinePercent = 3,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum CcConfigSweepMax CcConfigSweepMax;
#else
typedef uint16_t CcConfigSweepMax;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum CcConfigGridTie
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  CcConfigGridTie_Disabled = 0,
  CcConfigGridTie_Enabled = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum CcConfigGridTie CcConfigGridTie;
#else
typedef uint16_t CcConfigGridTie;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum CcConfigTempComp
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  CcConfigTempComp_Wide = 0,
  CcConfigTempComp_Limited = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum CcConfigTempComp CcConfigTempComp;
#else
typedef uint16_t CcConfigTempComp;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum CcConfigAutoRestart
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  CcConfigAutoRestart_Off = 0,
  CcConfigAutoRestart_Every90Minutes = 1,
  CcConfigAutoRestart_Every90MinutesIfAbsorbOrFloat = 2,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum CcConfigAutoRestart CcConfigAutoRestart;
#else
typedef uint16_t CcConfigAutoRestart;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum CcConfigAuxMode
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  CcConfigAuxMode_Float = 0,
  CcConfigAuxMode_DiversionRelay = 1,
  CcConfigAuxMode_DiversionSolidSt = 2,
  CcConfigAuxMode_LowBattDisconnect = 3,
  CcConfigAuxMode_Remote = 4,
  CcConfigAuxMode_VentFan = 5,
  CcConfigAuxMode_PvTrigger = 6,
  CcConfigAuxMode_ErrorOutput = 7,
  CcConfigAuxMode_NightLight = 8,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum CcConfigAuxMode CcConfigAuxMode;
#else
typedef uint16_t CcConfigAuxMode;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum CcConfigAuxControl
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  CcConfigAuxControl_Off = 0,
  CcConfigAuxControl_Auto = 1,
  CcConfigAuxControl_On = 2,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum CcConfigAuxControl CcConfigAuxControl;
#else
typedef uint16_t CcConfigAuxControl;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum CcConfigAuxState
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  CcConfigAuxState_Disabled = 0,
  CcConfigAuxState_Enabled = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum CcConfigAuxState CcConfigAuxState;
#else
typedef uint16_t CcConfigAuxState;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum CcConfigAuxPolarity
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  CcConfigAuxPolarity_Low = 0,
  CcConfigAuxPolarity_High = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum CcConfigAuxPolarity CcConfigAuxPolarity;
#else
typedef uint16_t CcConfigAuxPolarity;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum Mode
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   CV Mode
   
   Constant Voltage (CV) Mode
   */
  Mode_Cv = 0,
  /*
   CC Mode
   
   Constant Current (CC) Mode.
   */
  Mode_Cc = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum Mode Mode;
#else
typedef uint16_t Mode;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum Reset
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Reset Device
   
   Reset Device
   */
  Reset_Reset = 1,
  /*
   Do Not Reset Device
   
   Do Not Reset Device
   */
  Reset_DoNotReset = 0,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum Reset Reset;
#else
typedef uint16_t Reset;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum En50530
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   EN50530 Mode
   
   EN50530 Mode
   */
  En50530_En50530 = 1,
  /*
   Do Not Use EN50530 Mode
   
   Do Not Use EN50530 Mode
   */
  En50530_DoNotEn50530 = 0,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum En50530 En50530;
#else
typedef uint16_t En50530;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum EnaProf
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Start Profile
   
   Start the Profile
   */
  EnaProf_Start = 1,
  /*
   Stop Profile
   
   Stop the Profile
   */
  EnaProf_Stop = 0,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum EnaProf EnaProf;
#else
typedef uint16_t EnaProf;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum AdptProfRslt
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Update In Progress
   
   Profile update in progress.
   */
  AdptProfRslt_InProgress = 0,
  /*
   Update Complete
   
   Profile update completed successfully.
   */
  AdptProfRslt_Completed = 1,
  /*
   Update Failed
   
   Profile update failed.
   */
  AdptProfRslt_Failed = 2,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum AdptProfRslt AdptProfRslt;
#else
typedef uint16_t AdptProfRslt;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum Output
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Output Off
   */
  Output_Off = 0,
  /*
   Output On
   */
  Output_On = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum Output Output;
#else
typedef uint16_t Output;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum Relay
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Relay Open
   */
  Relay_Open = 0,
  /*
   Relay Closed
   */
  Relay_Closed = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum Relay Relay;
#else
typedef uint16_t Relay;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum Regen
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Regen Off
   */
  Regen_Off = 0,
  /*
   Regen On
   */
  Regen_On = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum Regen Regen;
#else
typedef uint16_t Regen;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum ProfRslt
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Profile update in progress.
   */
  ProfRslt_InProgress = 0,
  /*
   Profile update completed successfully.
   */
  ProfRslt_Completed = 1,
  /*
   Profile update failed.
   */
  ProfRslt_Failed = 2,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum ProfRslt ProfRslt;
#else
typedef uint16_t ProfRslt;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum DaManipulation
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Data Unaffected
   
   Modbus Falsification Disabled
   */
  DaManipulation_Off = 0,
  /*
   Data Falsification
   
   Modbus Falsification Enabled
   */
  DaManipulation_On = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum DaManipulation DaManipulation;
#else
typedef uint16_t DaManipulation;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum FalsifyDeviceIdentity
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Data Unaffected
   
   Modbus Falsification Disabled
   */
  FalsifyDeviceIdentity_Off = 0,
  /*
   Data Falsification
   
   Modbus Falsification Enabled
   */
  FalsifyDeviceIdentity_On = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum FalsifyDeviceIdentity FalsifyDeviceIdentity;
#else
typedef uint16_t FalsifyDeviceIdentity;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum MeasPAlwaysNameplate
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Data Unaffected
   
   Modbus Falsification Disabled
   */
  MeasPAlwaysNameplate_Off = 0,
  /*
   Data Falsification
   
   Modbus Falsification Enabled
   */
  MeasPAlwaysNameplate_On = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum MeasPAlwaysNameplate MeasPAlwaysNameplate;
#else
typedef uint16_t MeasPAlwaysNameplate;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum MeasQAlwaysMinimum
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Data Unaffected
   
   Modbus Falsification Disabled
   */
  MeasQAlwaysMinimum_Off = 0,
  /*
   Data Falsification
   
   Modbus Falsification Enabled
   */
  MeasQAlwaysMinimum_On = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum MeasQAlwaysMinimum MeasQAlwaysMinimum;
#else
typedef uint16_t MeasQAlwaysMinimum;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum MeasQAlwaysMaximum
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Data Unaffected
   
   Modbus Falsification Disabled
   */
  MeasQAlwaysMaximum_Off = 0,
  /*
   Data Falsification
   
   Modbus Falsification Enabled
   */
  MeasQAlwaysMaximum_On = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum MeasQAlwaysMaximum MeasQAlwaysMaximum;
#else
typedef uint16_t MeasQAlwaysMaximum;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum MeasQAlwaysZero
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Data Unaffected
   
   Modbus Falsification Disabled
   */
  MeasQAlwaysZero_Off = 0,
  /*
   Data Falsification
   
   Modbus Falsification Enabled
   */
  MeasQAlwaysZero_On = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum MeasQAlwaysZero MeasQAlwaysZero;
#else
typedef uint16_t MeasQAlwaysZero;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum MeasZeroP
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Data Unaffected
   
   Modbus Falsification Disabled
   */
  MeasZeroP_Off = 0,
  /*
   Data Falsification
   
   Modbus Falsification Enabled
   */
  MeasZeroP_On = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum MeasZeroP MeasZeroP;
#else
typedef uint16_t MeasZeroP;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum MeasInvertQ
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Data Unaffected
   
   Modbus Falsification Disabled
   */
  MeasInvertQ_Off = 0,
  /*
   Data Falsification
   
   Modbus Falsification Enabled
   */
  MeasInvertQ_On = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum MeasInvertQ MeasInvertQ;
#else
typedef uint16_t MeasInvertQ;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum MeasLowV
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Data Unaffected
   
   Modbus Falsification Disabled
   */
  MeasLowV_Off = 0,
  /*
   Data Falsification
   
   Modbus Falsification Enabled
   */
  MeasLowV_On = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum MeasLowV MeasLowV;
#else
typedef uint16_t MeasLowV;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum MeasHighV
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Data Unaffected
   
   Modbus Falsification Disabled
   */
  MeasHighV_Off = 0,
  /*
   Data Falsification
   
   Modbus Falsification Enabled
   */
  MeasHighV_On = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum MeasHighV MeasHighV;
#else
typedef uint16_t MeasHighV;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum MeasLowL1v
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Data Unaffected
   
   Modbus Falsification Disabled
   */
  MeasLowL1v_Off = 0,
  /*
   Data Falsification
   
   Modbus Falsification Enabled
   */
  MeasLowL1v_On = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum MeasLowL1v MeasLowL1v;
#else
typedef uint16_t MeasLowL1v;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum MeasHighL1v
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Data Unaffected
   
   Modbus Falsification Disabled
   */
  MeasHighL1v_Off = 0,
  /*
   Data Falsification
   
   Modbus Falsification Enabled
   */
  MeasHighL1v_On = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum MeasHighL1v MeasHighL1v;
#else
typedef uint16_t MeasHighL1v;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum MeasLowF
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Data Unaffected
   
   Modbus Falsification Disabled
   */
  MeasLowF_Off = 0,
  /*
   Data Falsification
   
   Modbus Falsification Enabled
   */
  MeasLowF_On = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum MeasLowF MeasLowF;
#else
typedef uint16_t MeasLowF;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum MeasHighF
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Data Unaffected
   
   Modbus Falsification Disabled
   */
  MeasHighF_Off = 0,
  /*
   Data Falsification
   
   Modbus Falsification Enabled
   */
  MeasHighF_On = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum MeasHighF MeasHighF;
#else
typedef uint16_t MeasHighF;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum MeasLowAmps
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Data Unaffected
   
   Modbus Falsification Disabled
   */
  MeasLowAmps_Off = 0,
  /*
   Data Falsification
   
   Modbus Falsification Enabled
   */
  MeasLowAmps_On = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum MeasLowAmps MeasLowAmps;
#else
typedef uint16_t MeasLowAmps;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum MeasHighAmps
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Data Unaffected
   
   Modbus Falsification Disabled
   */
  MeasHighAmps_Off = 0,
  /*
   Data Falsification
   
   Modbus Falsification Enabled
   */
  MeasHighAmps_On = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum MeasHighAmps MeasHighAmps;
#else
typedef uint16_t MeasHighAmps;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum MeasHighS
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Data Unaffected
   
   Modbus Falsification Disabled
   */
  MeasHighS_Off = 0,
  /*
   Data Falsification
   
   Modbus Falsification Enabled
   */
  MeasHighS_On = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum MeasHighS MeasHighS;
#else
typedef uint16_t MeasHighS;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum MeasLowS
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Data Unaffected
   
   Modbus Falsification Disabled
   */
  MeasLowS_Off = 0,
  /*
   Data Falsification
   
   Modbus Falsification Enabled
   */
  MeasLowS_On = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum MeasLowS MeasLowS;
#else
typedef uint16_t MeasLowS;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum MeasHighQ
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Data Unaffected
   
   Modbus Falsification Disabled
   */
  MeasHighQ_Off = 0,
  /*
   Data Falsification
   
   Modbus Falsification Enabled
   */
  MeasHighQ_On = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum MeasHighQ MeasHighQ;
#else
typedef uint16_t MeasHighQ;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum MeasLowQ
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Data Unaffected
   
   Modbus Falsification Disabled
   */
  MeasLowQ_Off = 0,
  /*
   Data Falsification
   
   Modbus Falsification Enabled
   */
  MeasLowQ_On = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum MeasLowQ MeasLowQ;
#else
typedef uint16_t MeasLowQ;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum MeasLowPf
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Data Unaffected
   
   Modbus Falsification Disabled
   */
  MeasLowPf_Off = 0,
  /*
   Data Falsification
   
   Modbus Falsification Enabled
   */
  MeasLowPf_On = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum MeasLowPf MeasLowPf;
#else
typedef uint16_t MeasLowPf;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum MeasLowReversedPf
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Data Unaffected
   
   Modbus Falsification Disabled
   */
  MeasLowReversedPf_Off = 0,
  /*
   Data Falsification
   
   Modbus Falsification Enabled
   */
  MeasLowReversedPf_On = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum MeasLowReversedPf MeasLowReversedPf;
#else
typedef uint16_t MeasLowReversedPf;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum NameplateHighP
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Data Unaffected
   
   Modbus Falsification Disabled
   */
  NameplateHighP_Off = 0,
  /*
   Data Falsification
   
   Modbus Falsification Enabled
   */
  NameplateHighP_On = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum NameplateHighP NameplateHighP;
#else
typedef uint16_t NameplateHighP;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum NameplateLowP
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Data Unaffected
   
   Modbus Falsification Disabled
   */
  NameplateLowP_Off = 0,
  /*
   Data Falsification
   
   Modbus Falsification Enabled
   */
  NameplateLowP_On = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum NameplateLowP NameplateLowP;
#else
typedef uint16_t NameplateLowP;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum NameplateHighS
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Data Unaffected
   
   Modbus Falsification Disabled
   */
  NameplateHighS_Off = 0,
  /*
   Data Falsification
   
   Modbus Falsification Enabled
   */
  NameplateHighS_On = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum NameplateHighS NameplateHighS;
#else
typedef uint16_t NameplateHighS;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum NameplateLowS
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Data Unaffected
   
   Modbus Falsification Disabled
   */
  NameplateLowS_Off = 0,
  /*
   Data Falsification
   
   Modbus Falsification Enabled
   */
  NameplateLowS_On = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum NameplateLowS NameplateLowS;
#else
typedef uint16_t NameplateLowS;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum NameplateHighQ
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Data Unaffected
   
   Modbus Falsification Disabled
   */
  NameplateHighQ_Off = 0,
  /*
   Data Falsification
   
   Modbus Falsification Enabled
   */
  NameplateHighQ_On = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum NameplateHighQ NameplateHighQ;
#else
typedef uint16_t NameplateHighQ;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum NameplateLowQ
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Data Unaffected
   
   Modbus Falsification Disabled
   */
  NameplateLowQ_Off = 0,
  /*
   Data Falsification
   
   Modbus Falsification Enabled
   */
  NameplateLowQ_On = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum NameplateLowQ NameplateLowQ;
#else
typedef uint16_t NameplateLowQ;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum NameplateHighNomV
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Data Unaffected
   
   Modbus Falsification Disabled
   */
  NameplateHighNomV_Off = 0,
  /*
   Data Falsification
   
   Modbus Falsification Enabled
   */
  NameplateHighNomV_On = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum NameplateHighNomV NameplateHighNomV;
#else
typedef uint16_t NameplateHighNomV;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum NameplateLowNomV
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Data Unaffected
   
   Modbus Falsification Disabled
   */
  NameplateLowNomV_Off = 0,
  /*
   Data Falsification
   
   Modbus Falsification Enabled
   */
  NameplateLowNomV_On = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum NameplateLowNomV NameplateLowNomV;
#else
typedef uint16_t NameplateLowNomV;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum NameplateLowAmps
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Data Unaffected
   
   Modbus Falsification Disabled
   */
  NameplateLowAmps_Off = 0,
  /*
   Data Falsification
   
   Modbus Falsification Enabled
   */
  NameplateLowAmps_On = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum NameplateLowAmps NameplateLowAmps;
#else
typedef uint16_t NameplateLowAmps;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum NameplateLowVarmaxinj
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Data Unaffected
   
   Modbus Falsification Disabled
   */
  NameplateLowVarmaxinj_Off = 0,
  /*
   Data Falsification
   
   Modbus Falsification Enabled
   */
  NameplateLowVarmaxinj_On = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum NameplateLowVarmaxinj NameplateLowVarmaxinj;
#else
typedef uint16_t NameplateLowVarmaxinj;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum NameplateLowVarmaxabs
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Data Unaffected
   
   Modbus Falsification Disabled
   */
  NameplateLowVarmaxabs_Off = 0,
  /*
   Data Falsification
   
   Modbus Falsification Enabled
   */
  NameplateLowVarmaxabs_On = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum NameplateLowVarmaxabs NameplateLowVarmaxabs;
#else
typedef uint16_t NameplateLowVarmaxabs;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum NameplateLowPf
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Data Unaffected
   
   Modbus Falsification Disabled
   */
  NameplateLowPf_Off = 0,
  /*
   Data Falsification
   
   Modbus Falsification Enabled
   */
  NameplateLowPf_On = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum NameplateLowPf NameplateLowPf;
#else
typedef uint16_t NameplateLowPf;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum SettingsHighNomV
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Data Unaffected
   
   Modbus Falsification Disabled
   */
  SettingsHighNomV_Off = 0,
  /*
   Data Falsification
   
   Modbus Falsification Enabled
   */
  SettingsHighNomV_On = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum SettingsHighNomV SettingsHighNomV;
#else
typedef uint16_t SettingsHighNomV;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum SettingsLowAmps
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Data Unaffected
   
   Modbus Falsification Disabled
   */
  SettingsLowAmps_Off = 0,
  /*
   Data Falsification
   
   Modbus Falsification Enabled
   */
  SettingsLowAmps_On = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum SettingsLowAmps SettingsLowAmps;
#else
typedef uint16_t SettingsLowAmps;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum SettingsHighP
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Data Unaffected
   
   Modbus Falsification Disabled
   */
  SettingsHighP_Off = 0,
  /*
   Data Falsification
   
   Modbus Falsification Enabled
   */
  SettingsHighP_On = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum SettingsHighP SettingsHighP;
#else
typedef uint16_t SettingsHighP;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum SettingsLowP
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Data Unaffected
   
   Modbus Falsification Disabled
   */
  SettingsLowP_Off = 0,
  /*
   Data Falsification
   
   Modbus Falsification Enabled
   */
  SettingsLowP_On = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum SettingsLowP SettingsLowP;
#else
typedef uint16_t SettingsLowP;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum SettingsHighVaMax
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Data Unaffected
   
   Modbus Falsification Disabled
   */
  SettingsHighVaMax_Off = 0,
  /*
   Data Falsification
   
   Modbus Falsification Enabled
   */
  SettingsHighVaMax_On = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum SettingsHighVaMax SettingsHighVaMax;
#else
typedef uint16_t SettingsHighVaMax;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum SettingsHighVarmaxinj
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Data Unaffected
   
   Modbus Falsification Disabled
   */
  SettingsHighVarmaxinj_Off = 0,
  /*
   Data Falsification
   
   Modbus Falsification Enabled
   */
  SettingsHighVarmaxinj_On = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum SettingsHighVarmaxinj SettingsHighVarmaxinj;
#else
typedef uint16_t SettingsHighVarmaxinj;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum SettingsHighVarmaxabs
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Data Unaffected
   
   Modbus Falsification Disabled
   */
  SettingsHighVarmaxabs_Off = 0,
  /*
   Data Falsification
   
   Modbus Falsification Enabled
   */
  SettingsHighVarmaxabs_On = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum SettingsHighVarmaxabs SettingsHighVarmaxabs;
#else
typedef uint16_t SettingsHighVarmaxabs;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum ChangeCommonModelId
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Data Unaffected
   
   Modbus Falsification Disabled
   */
  ChangeCommonModelId_Off = 0,
  /*
   Data Falsification
   
   Modbus Falsification Enabled
   */
  ChangeCommonModelId_On = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum ChangeCommonModelId ChangeCommonModelId;
#else
typedef uint16_t ChangeCommonModelId;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum ChangeCommonModelLength
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Data Unaffected
   
   Modbus Falsification Disabled
   */
  ChangeCommonModelLength_Off = 0,
  /*
   Data Falsification
   
   Modbus Falsification Enabled
   */
  ChangeCommonModelLength_On = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum ChangeCommonModelLength ChangeCommonModelLength;
#else
typedef uint16_t ChangeCommonModelLength;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum LogEventEna
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Disabled
   
   LogEvent Mode Disabled
   */
  LogEventEna_Disabled = 0,
  /*
   Enabled
   
   LogEvent Mode Enabled
   */
  LogEventEna_Enabled = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum LogEventEna LogEventEna;
#else
typedef uint16_t LogEventEna;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum HttpMsg
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Disabled
   
   HTTP Message Mode Disabled
   */
  HttpMsg_Disabled = 0,
  /*
   Enabled
   
   HTTP Message Mode Enabled
   */
  HttpMsg_Enabled = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum HttpMsg HttpMsg;
#else
typedef uint16_t HttpMsg;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum Comm004Cert
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   DEFAULT
   
   Default Certificate
   */
  Comm004Cert_DefaultCertificate = 0,
  /*
   COMM-004A
   
   Chain Length Two Certificate
   */
  Comm004Cert_Comm004a = 1,
  /*
   COMM-004B
   
   Chain Length Three Certificate
   */
  Comm004Cert_Comm004b = 2,
  /*
   COMM-004C
   
   Chain Length Four Certificate
   */
  Comm004Cert_Comm004c = 3,
  /*
   COMM-004D
   
   Invalid MICA Extended Key Critical Value
   */
  Comm004Cert_Comm004d = 4,
  /*
   COMM-004E
   
   Invalid MICA Name Non-Critical Value
   */
  Comm004Cert_Comm004e = 5,
  /*
   COMM-004F
   
   Invalid MICA Policy Mapping Non-Critical Value
   */
  Comm004Cert_Comm004f = 6,
  /*
   COMM-004G
   
   Self-signed device certificate
   */
  Comm004Cert_Comm004g = 7,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum Comm004Cert Comm004Cert;
#else
typedef uint16_t Comm004Cert;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum SubscriptionEna
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Disabled
   
   Subscription Mode Disabled
   */
  SubscriptionEna_Disabled = 0,
  /*
   Enabled
   
   Subscription Mode Enabled
   */
  SubscriptionEna_Enabled = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum SubscriptionEna SubscriptionEna;
#else
typedef uint16_t SubscriptionEna;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

typedef struct Model1CallbackAdapter {
  void *context;
  const char *(*manufacturer_callback)(const void*);
  const char *(*model_callback)(const void*);
  const char *(*options_callback)(const void*);
  const char *(*version_callback)(const void*);
  const char *(*serial_number_callback)(const void*);
  uint16_t (*device_address_callback)(const void*);
  void (*set_device_address_callback)(uint16_t, void*);
} Model1CallbackAdapter;

typedef struct Model1StatefulAdapter {
  char manufacturer[32];
  char model[32];
  char options[16];
  char version[16];
  char serial_number[32];
  uint16_t device_address;
} Model1StatefulAdapter;

typedef struct Model2CallbackAdapter {
  void *context;
  uint16_t (*aid_callback)(const void*);
  uint16_t (*n_callback)(const void*);
  uint16_t (*un_callback)(const void*);
  St (*status_callback)(const void*);
  uint16_t (*vendor_status_callback)(const void*);
  uint32_t (*event_code_callback)(const void*);
  uint32_t (*vendor_event_code_callback)(const void*);
  Ctl (*control_callback)(const void*);
  uint32_t (*vendor_control_callback)(const void*);
  uint32_t (*control_value_callback)(const void*);
} Model2CallbackAdapter;

typedef struct Model2StatefulAdapter {
  uint16_t aid;
  uint16_t n;
  uint16_t un;
  St status;
  uint16_t vendor_status;
  uint32_t event_code;
  uint32_t vendor_event_code;
  Ctl control;
  uint32_t vendor_control;
  uint32_t control_value;
} Model2StatefulAdapter;

typedef struct Model3CallbackAdapter {
  void *context;
  uint16_t (*x_callback)(const void*);
  void (*set_x_callback)(uint16_t, void*);
  uint16_t (*offset1_callback)(const void*);
  void (*set_offset1_callback)(uint16_t, void*);
  uint16_t (*off2_callback)(const void*);
  void (*set_off2_callback)(uint16_t, void*);
  uint16_t (*off3_callback)(const void*);
  void (*set_off3_callback)(uint16_t, void*);
  uint16_t (*off4_callback)(const void*);
  void (*set_off4_callback)(uint16_t, void*);
  uint16_t (*off5_callback)(const void*);
  void (*set_off5_callback)(uint16_t, void*);
  uint16_t (*off6_callback)(const void*);
  void (*set_off6_callback)(uint16_t, void*);
  uint16_t (*off7_callback)(const void*);
  void (*set_off7_callback)(uint16_t, void*);
  uint16_t (*off8_callback)(const void*);
  void (*set_off8_callback)(uint16_t, void*);
  uint16_t (*off9_callback)(const void*);
  void (*set_off9_callback)(uint16_t, void*);
  uint16_t (*off10_callback)(const void*);
  void (*set_off10_callback)(uint16_t, void*);
  uint16_t (*off11_callback)(const void*);
  void (*set_off11_callback)(uint16_t, void*);
  uint16_t (*off12_callback)(const void*);
  void (*set_off12_callback)(uint16_t, void*);
  uint16_t (*off13_callback)(const void*);
  void (*set_off13_callback)(uint16_t, void*);
  uint16_t (*off14_callback)(const void*);
  void (*set_off14_callback)(uint16_t, void*);
  uint16_t (*off15_callback)(const void*);
  void (*set_off15_callback)(uint16_t, void*);
  uint16_t (*off16_callback)(const void*);
  void (*set_off16_callback)(uint16_t, void*);
  uint16_t (*off17_callback)(const void*);
  void (*set_off17_callback)(uint16_t, void*);
  uint16_t (*off18_callback)(const void*);
  void (*set_off18_callback)(uint16_t, void*);
  uint16_t (*off19_callback)(const void*);
  void (*set_off19_callback)(uint16_t, void*);
  uint16_t (*off20_callback)(const void*);
  void (*set_off20_callback)(uint16_t, void*);
  uint16_t (*off21_callback)(const void*);
  void (*set_off21_callback)(uint16_t, void*);
  uint16_t (*off22_callback)(const void*);
  void (*set_off22_callback)(uint16_t, void*);
  uint16_t (*off23_callback)(const void*);
  void (*set_off23_callback)(uint16_t, void*);
  uint16_t (*off24_callback)(const void*);
  void (*set_off24_callback)(uint16_t, void*);
  uint16_t (*off25_callback)(const void*);
  void (*set_off25_callback)(uint16_t, void*);
  uint16_t (*off26_callback)(const void*);
  void (*set_off26_callback)(uint16_t, void*);
  uint16_t (*off27_callback)(const void*);
  void (*set_off27_callback)(uint16_t, void*);
  uint16_t (*off28_callback)(const void*);
  void (*set_off28_callback)(uint16_t, void*);
  uint16_t (*off29_callback)(const void*);
  void (*set_off29_callback)(uint16_t, void*);
  uint16_t (*off30_callback)(const void*);
  void (*set_off30_callback)(uint16_t, void*);
  uint16_t (*off31_callback)(const void*);
  void (*set_off31_callback)(uint16_t, void*);
  uint16_t (*off32_callback)(const void*);
  void (*set_off32_callback)(uint16_t, void*);
  uint16_t (*off33_callback)(const void*);
  void (*set_off33_callback)(uint16_t, void*);
  uint16_t (*off34_callback)(const void*);
  void (*set_off34_callback)(uint16_t, void*);
  uint16_t (*off35_callback)(const void*);
  void (*set_off35_callback)(uint16_t, void*);
  uint16_t (*off36_callback)(const void*);
  void (*set_off36_callback)(uint16_t, void*);
  uint16_t (*off37_callback)(const void*);
  void (*set_off37_callback)(uint16_t, void*);
  uint16_t (*off38_callback)(const void*);
  void (*set_off38_callback)(uint16_t, void*);
  uint16_t (*off39_callback)(const void*);
  void (*set_off39_callback)(uint16_t, void*);
  uint16_t (*off40_callback)(const void*);
  void (*set_off40_callback)(uint16_t, void*);
  uint16_t (*off41_callback)(const void*);
  void (*set_off41_callback)(uint16_t, void*);
  uint16_t (*off42_callback)(const void*);
  void (*set_off42_callback)(uint16_t, void*);
  uint16_t (*off43_callback)(const void*);
  void (*set_off43_callback)(uint16_t, void*);
  uint16_t (*off44_callback)(const void*);
  void (*set_off44_callback)(uint16_t, void*);
  uint16_t (*off45_callback)(const void*);
  void (*set_off45_callback)(uint16_t, void*);
  uint16_t (*off46_callback)(const void*);
  void (*set_off46_callback)(uint16_t, void*);
  uint16_t (*off47_callback)(const void*);
  void (*set_off47_callback)(uint16_t, void*);
  uint16_t (*off48_callback)(const void*);
  void (*set_off48_callback)(uint16_t, void*);
  uint16_t (*off49_callback)(const void*);
  void (*set_off49_callback)(uint16_t, void*);
  uint16_t (*off50_callback)(const void*);
  void (*set_off50_callback)(uint16_t, void*);
  uint32_t (*timestamp_callback)(const void*);
  void (*set_timestamp_callback)(uint32_t, void*);
  uint16_t (*milliseconds_callback)(const void*);
  void (*set_milliseconds_callback)(uint16_t, void*);
  uint16_t (*sequence_callback)(const void*);
  void (*set_sequence_callback)(uint16_t, void*);
  uint16_t (*role_callback)(const void*);
  void (*set_role_callback)(uint16_t, void*);
  Alg (*algorithm_callback)(const void*);
  uint16_t (*n_callback)(const void*);
} Model3CallbackAdapter;

typedef struct Model3StatefulAdapter {
  uint16_t x;
  uint16_t offset1;
  uint16_t off2;
  uint16_t off3;
  uint16_t off4;
  uint16_t off5;
  uint16_t off6;
  uint16_t off7;
  uint16_t off8;
  uint16_t off9;
  uint16_t off10;
  uint16_t off11;
  uint16_t off12;
  uint16_t off13;
  uint16_t off14;
  uint16_t off15;
  uint16_t off16;
  uint16_t off17;
  uint16_t off18;
  uint16_t off19;
  uint16_t off20;
  uint16_t off21;
  uint16_t off22;
  uint16_t off23;
  uint16_t off24;
  uint16_t off25;
  uint16_t off26;
  uint16_t off27;
  uint16_t off28;
  uint16_t off29;
  uint16_t off30;
  uint16_t off31;
  uint16_t off32;
  uint16_t off33;
  uint16_t off34;
  uint16_t off35;
  uint16_t off36;
  uint16_t off37;
  uint16_t off38;
  uint16_t off39;
  uint16_t off40;
  uint16_t off41;
  uint16_t off42;
  uint16_t off43;
  uint16_t off44;
  uint16_t off45;
  uint16_t off46;
  uint16_t off47;
  uint16_t off48;
  uint16_t off49;
  uint16_t off50;
  uint32_t timestamp;
  uint16_t milliseconds;
  uint16_t sequence;
  uint16_t role;
  Alg algorithm;
  uint16_t n;
} Model3StatefulAdapter;

typedef struct Model4CallbackAdapter {
  void *context;
  uint16_t (*request_sequence_callback)(const void*);
  Sts (*status_callback)(const void*);
  uint16_t (*x_callback)(const void*);
  uint16_t (*value1_callback)(const void*);
  uint16_t (*val2_callback)(const void*);
  uint16_t (*val3_callback)(const void*);
  uint16_t (*val4_callback)(const void*);
  uint16_t (*val5_callback)(const void*);
  uint16_t (*val6_callback)(const void*);
  uint16_t (*val7_callback)(const void*);
  uint16_t (*val8_callback)(const void*);
  uint16_t (*val9_callback)(const void*);
  uint16_t (*val10_callback)(const void*);
  uint16_t (*val11_callback)(const void*);
  uint16_t (*val12_callback)(const void*);
  uint16_t (*val13_callback)(const void*);
  uint16_t (*val14_callback)(const void*);
  uint16_t (*val15_callback)(const void*);
  uint16_t (*val16_callback)(const void*);
  uint16_t (*val17_callback)(const void*);
  uint16_t (*val18_callback)(const void*);
  uint16_t (*val19_callback)(const void*);
  uint16_t (*val20_callback)(const void*);
  uint16_t (*val21_callback)(const void*);
  uint16_t (*val22_callback)(const void*);
  uint16_t (*val23_callback)(const void*);
  uint16_t (*val24_callback)(const void*);
  uint16_t (*val25_callback)(const void*);
  uint16_t (*val26_callback)(const void*);
  uint16_t (*val27_callback)(const void*);
  uint16_t (*val28_callback)(const void*);
  uint16_t (*val29_callback)(const void*);
  uint16_t (*val30_callback)(const void*);
  uint16_t (*val31_callback)(const void*);
  uint16_t (*val32_callback)(const void*);
  uint16_t (*val33_callback)(const void*);
  uint16_t (*val34_callback)(const void*);
  uint16_t (*val35_callback)(const void*);
  uint16_t (*val36_callback)(const void*);
  uint16_t (*val37_callback)(const void*);
  uint16_t (*val38_callback)(const void*);
  uint16_t (*val39_callback)(const void*);
  uint16_t (*val40_callback)(const void*);
  uint16_t (*val41_callback)(const void*);
  uint16_t (*val42_callback)(const void*);
  uint16_t (*val43_callback)(const void*);
  uint16_t (*val44_callback)(const void*);
  uint16_t (*val45_callback)(const void*);
  uint16_t (*val46_callback)(const void*);
  uint16_t (*val47_callback)(const void*);
  uint16_t (*val48_callback)(const void*);
  uint16_t (*val49_callback)(const void*);
  uint16_t (*val50_callback)(const void*);
  uint32_t (*timestamp_callback)(const void*);
  uint16_t (*milliseconds_callback)(const void*);
  uint16_t (*sequence_callback)(const void*);
  Alm (*alarm_callback)(const void*);
  Alg (*algorithm_callback)(const void*);
  uint16_t (*n_callback)(const void*);
} Model4CallbackAdapter;

typedef struct Model4StatefulAdapter {
  uint16_t request_sequence;
  Sts status;
  uint16_t x;
  uint16_t value1;
  uint16_t val2;
  uint16_t val3;
  uint16_t val4;
  uint16_t val5;
  uint16_t val6;
  uint16_t val7;
  uint16_t val8;
  uint16_t val9;
  uint16_t val10;
  uint16_t val11;
  uint16_t val12;
  uint16_t val13;
  uint16_t val14;
  uint16_t val15;
  uint16_t val16;
  uint16_t val17;
  uint16_t val18;
  uint16_t val19;
  uint16_t val20;
  uint16_t val21;
  uint16_t val22;
  uint16_t val23;
  uint16_t val24;
  uint16_t val25;
  uint16_t val26;
  uint16_t val27;
  uint16_t val28;
  uint16_t val29;
  uint16_t val30;
  uint16_t val31;
  uint16_t val32;
  uint16_t val33;
  uint16_t val34;
  uint16_t val35;
  uint16_t val36;
  uint16_t val37;
  uint16_t val38;
  uint16_t val39;
  uint16_t val40;
  uint16_t val41;
  uint16_t val42;
  uint16_t val43;
  uint16_t val44;
  uint16_t val45;
  uint16_t val46;
  uint16_t val47;
  uint16_t val48;
  uint16_t val49;
  uint16_t val50;
  uint32_t timestamp;
  uint16_t milliseconds;
  uint16_t sequence;
  Alm alarm;
  Alg algorithm;
  uint16_t n;
} Model4StatefulAdapter;

typedef struct Model5CallbackAdapter {
  void *context;
  uint16_t (*x_callback)(const void*);
  void (*set_x_callback)(uint16_t, void*);
  uint16_t (*offset1_callback)(const void*);
  void (*set_offset1_callback)(uint16_t, void*);
  uint16_t (*value1_callback)(const void*);
  void (*set_value1_callback)(uint16_t, void*);
  uint16_t (*off2_callback)(const void*);
  void (*set_off2_callback)(uint16_t, void*);
  uint16_t (*val2_callback)(const void*);
  void (*set_val2_callback)(uint16_t, void*);
  uint16_t (*off3_callback)(const void*);
  void (*set_off3_callback)(uint16_t, void*);
  uint16_t (*val3_callback)(const void*);
  void (*set_val3_callback)(uint16_t, void*);
  uint16_t (*off4_callback)(const void*);
  void (*set_off4_callback)(uint16_t, void*);
  uint16_t (*val4_callback)(const void*);
  void (*set_val4_callback)(uint16_t, void*);
  uint16_t (*off5_callback)(const void*);
  void (*set_off5_callback)(uint16_t, void*);
  uint16_t (*val5_callback)(const void*);
  void (*set_val5_callback)(uint16_t, void*);
  uint16_t (*off6_callback)(const void*);
  void (*set_off6_callback)(uint16_t, void*);
  uint16_t (*val6_callback)(const void*);
  void (*set_val6_callback)(uint16_t, void*);
  uint16_t (*off7_callback)(const void*);
  void (*set_off7_callback)(uint16_t, void*);
  uint16_t (*val7_callback)(const void*);
  void (*set_val7_callback)(uint16_t, void*);
  uint16_t (*off8_callback)(const void*);
  void (*set_off8_callback)(uint16_t, void*);
  uint16_t (*val8_callback)(const void*);
  void (*set_val8_callback)(uint16_t, void*);
  uint16_t (*off9_callback)(const void*);
  void (*set_off9_callback)(uint16_t, void*);
  uint16_t (*val9_callback)(const void*);
  void (*set_val9_callback)(uint16_t, void*);
  uint16_t (*off10_callback)(const void*);
  void (*set_off10_callback)(uint16_t, void*);
  uint16_t (*val10_callback)(const void*);
  void (*set_val10_callback)(uint16_t, void*);
  uint16_t (*off11_callback)(const void*);
  void (*set_off11_callback)(uint16_t, void*);
  uint16_t (*val11_callback)(const void*);
  void (*set_val11_callback)(uint16_t, void*);
  uint16_t (*off12_callback)(const void*);
  void (*set_off12_callback)(uint16_t, void*);
  uint16_t (*val12_callback)(const void*);
  void (*set_val12_callback)(uint16_t, void*);
  uint16_t (*off13_callback)(const void*);
  void (*set_off13_callback)(uint16_t, void*);
  uint16_t (*val13_callback)(const void*);
  void (*set_val13_callback)(uint16_t, void*);
  uint16_t (*off14_callback)(const void*);
  void (*set_off14_callback)(uint16_t, void*);
  uint16_t (*val14_callback)(const void*);
  void (*set_val14_callback)(uint16_t, void*);
  uint16_t (*off15_callback)(const void*);
  void (*set_off15_callback)(uint16_t, void*);
  uint16_t (*val15_callback)(const void*);
  void (*set_val15_callback)(uint16_t, void*);
  uint16_t (*off16_callback)(const void*);
  void (*set_off16_callback)(uint16_t, void*);
  uint16_t (*val16_callback)(const void*);
  void (*set_val16_callback)(uint16_t, void*);
  uint16_t (*off17_callback)(const void*);
  void (*set_off17_callback)(uint16_t, void*);
  uint16_t (*val17_callback)(const void*);
  void (*set_val17_callback)(uint16_t, void*);
  uint16_t (*off18_callback)(const void*);
  void (*set_off18_callback)(uint16_t, void*);
  uint16_t (*val18_callback)(const void*);
  void (*set_val18_callback)(uint16_t, void*);
  uint16_t (*off19_callback)(const void*);
  void (*set_off19_callback)(uint16_t, void*);
  uint16_t (*val19_callback)(const void*);
  void (*set_val19_callback)(uint16_t, void*);
  uint16_t (*off20_callback)(const void*);
  void (*set_off20_callback)(uint16_t, void*);
  uint16_t (*val20_callback)(const void*);
  void (*set_val20_callback)(uint16_t, void*);
  uint16_t (*off21_callback)(const void*);
  void (*set_off21_callback)(uint16_t, void*);
  uint16_t (*val21_callback)(const void*);
  void (*set_val21_callback)(uint16_t, void*);
  uint16_t (*off22_callback)(const void*);
  void (*set_off22_callback)(uint16_t, void*);
  uint16_t (*val22_callback)(const void*);
  void (*set_val22_callback)(uint16_t, void*);
  uint16_t (*off23_callback)(const void*);
  void (*set_off23_callback)(uint16_t, void*);
  uint16_t (*val23_callback)(const void*);
  void (*set_val23_callback)(uint16_t, void*);
  uint16_t (*off24_callback)(const void*);
  void (*set_off24_callback)(uint16_t, void*);
  uint16_t (*val24_callback)(const void*);
  void (*set_val24_callback)(uint16_t, void*);
  uint16_t (*off25_callback)(const void*);
  void (*set_off25_callback)(uint16_t, void*);
  uint16_t (*val25_callback)(const void*);
  void (*set_val25_callback)(uint16_t, void*);
  uint16_t (*off26_callback)(const void*);
  void (*set_off26_callback)(uint16_t, void*);
  uint16_t (*val26_callback)(const void*);
  void (*set_val26_callback)(uint16_t, void*);
  uint16_t (*off27_callback)(const void*);
  void (*set_off27_callback)(uint16_t, void*);
  uint16_t (*val27_callback)(const void*);
  void (*set_val27_callback)(uint16_t, void*);
  uint16_t (*off28_callback)(const void*);
  void (*set_off28_callback)(uint16_t, void*);
  uint16_t (*val28_callback)(const void*);
  void (*set_val28_callback)(uint16_t, void*);
  uint16_t (*off29_callback)(const void*);
  void (*set_off29_callback)(uint16_t, void*);
  uint16_t (*val29_callback)(const void*);
  void (*set_val29_callback)(uint16_t, void*);
  uint16_t (*off30_callback)(const void*);
  void (*set_off30_callback)(uint16_t, void*);
  uint16_t (*val30_callback)(const void*);
  void (*set_val30_callback)(uint16_t, void*);
  uint16_t (*off31_callback)(const void*);
  void (*set_off31_callback)(uint16_t, void*);
  uint16_t (*val31_callback)(const void*);
  void (*set_val31_callback)(uint16_t, void*);
  uint16_t (*off32_callback)(const void*);
  void (*set_off32_callback)(uint16_t, void*);
  uint16_t (*val32_callback)(const void*);
  void (*set_val32_callback)(uint16_t, void*);
  uint16_t (*off33_callback)(const void*);
  void (*set_off33_callback)(uint16_t, void*);
  uint16_t (*val33_callback)(const void*);
  void (*set_val33_callback)(uint16_t, void*);
  uint16_t (*off34_callback)(const void*);
  void (*set_off34_callback)(uint16_t, void*);
  uint16_t (*val34_callback)(const void*);
  void (*set_val34_callback)(uint16_t, void*);
  uint16_t (*off35_callback)(const void*);
  void (*set_off35_callback)(uint16_t, void*);
  uint16_t (*val35_callback)(const void*);
  void (*set_val35_callback)(uint16_t, void*);
  uint16_t (*off36_callback)(const void*);
  void (*set_off36_callback)(uint16_t, void*);
  uint16_t (*val36_callback)(const void*);
  void (*set_val36_callback)(uint16_t, void*);
  uint16_t (*off37_callback)(const void*);
  void (*set_off37_callback)(uint16_t, void*);
  uint16_t (*val37_callback)(const void*);
  void (*set_val37_callback)(uint16_t, void*);
  uint16_t (*off38_callback)(const void*);
  void (*set_off38_callback)(uint16_t, void*);
  uint16_t (*val38_callback)(const void*);
  void (*set_val38_callback)(uint16_t, void*);
  uint16_t (*off39_callback)(const void*);
  void (*set_off39_callback)(uint16_t, void*);
  uint16_t (*val39_callback)(const void*);
  void (*set_val39_callback)(uint16_t, void*);
  uint16_t (*off40_callback)(const void*);
  void (*set_off40_callback)(uint16_t, void*);
  uint16_t (*val40_callback)(const void*);
  void (*set_val40_callback)(uint16_t, void*);
  uint32_t (*timestamp_callback)(const void*);
  void (*set_timestamp_callback)(uint32_t, void*);
  uint16_t (*milliseconds_callback)(const void*);
  void (*set_milliseconds_callback)(uint16_t, void*);
  uint16_t (*sequence_callback)(const void*);
  void (*set_sequence_callback)(uint16_t, void*);
  uint16_t (*role_callback)(const void*);
  void (*set_role_callback)(uint16_t, void*);
  Alg (*algorithm_callback)(const void*);
  void (*set_algorithm_callback)(Alg, void*);
  uint16_t (*n_callback)(const void*);
  void (*set_n_callback)(uint16_t, void*);
} Model5CallbackAdapter;

typedef struct Model5StatefulAdapter {
  uint16_t x;
  uint16_t offset1;
  uint16_t value1;
  uint16_t off2;
  uint16_t val2;
  uint16_t off3;
  uint16_t val3;
  uint16_t off4;
  uint16_t val4;
  uint16_t off5;
  uint16_t val5;
  uint16_t off6;
  uint16_t val6;
  uint16_t off7;
  uint16_t val7;
  uint16_t off8;
  uint16_t val8;
  uint16_t off9;
  uint16_t val9;
  uint16_t off10;
  uint16_t val10;
  uint16_t off11;
  uint16_t val11;
  uint16_t off12;
  uint16_t val12;
  uint16_t off13;
  uint16_t val13;
  uint16_t off14;
  uint16_t val14;
  uint16_t off15;
  uint16_t val15;
  uint16_t off16;
  uint16_t val16;
  uint16_t off17;
  uint16_t val17;
  uint16_t off18;
  uint16_t val18;
  uint16_t off19;
  uint16_t val19;
  uint16_t off20;
  uint16_t val20;
  uint16_t off21;
  uint16_t val21;
  uint16_t off22;
  uint16_t val22;
  uint16_t off23;
  uint16_t val23;
  uint16_t off24;
  uint16_t val24;
  uint16_t off25;
  uint16_t val25;
  uint16_t off26;
  uint16_t val26;
  uint16_t off27;
  uint16_t val27;
  uint16_t off28;
  uint16_t val28;
  uint16_t off29;
  uint16_t val29;
  uint16_t off30;
  uint16_t val30;
  uint16_t off31;
  uint16_t val31;
  uint16_t off32;
  uint16_t val32;
  uint16_t off33;
  uint16_t val33;
  uint16_t off34;
  uint16_t val34;
  uint16_t off35;
  uint16_t val35;
  uint16_t off36;
  uint16_t val36;
  uint16_t off37;
  uint16_t val37;
  uint16_t off38;
  uint16_t val38;
  uint16_t off39;
  uint16_t val39;
  uint16_t off40;
  uint16_t val40;
  uint32_t timestamp;
  uint16_t milliseconds;
  uint16_t sequence;
  uint16_t role;
  Alg algorithm;
  uint16_t n;
} Model5StatefulAdapter;

typedef struct Model6CallbackAdapter {
  void *context;
  uint16_t (*x_callback)(const void*);
  void (*set_x_callback)(uint16_t, void*);
  uint16_t (*offset_callback)(const void*);
  void (*set_offset_callback)(uint16_t, void*);
  uint16_t (*value1_callback)(const void*);
  void (*set_value1_callback)(uint16_t, void*);
  uint16_t (*val2_callback)(const void*);
  void (*set_val2_callback)(uint16_t, void*);
  uint16_t (*val3_callback)(const void*);
  void (*set_val3_callback)(uint16_t, void*);
  uint16_t (*val4_callback)(const void*);
  void (*set_val4_callback)(uint16_t, void*);
  uint16_t (*val5_callback)(const void*);
  void (*set_val5_callback)(uint16_t, void*);
  uint16_t (*val6_callback)(const void*);
  void (*set_val6_callback)(uint16_t, void*);
  uint16_t (*val7_callback)(const void*);
  void (*set_val7_callback)(uint16_t, void*);
  uint16_t (*val8_callback)(const void*);
  void (*set_val8_callback)(uint16_t, void*);
  uint16_t (*val9_callback)(const void*);
  void (*set_val9_callback)(uint16_t, void*);
  uint16_t (*val10_callback)(const void*);
  void (*set_val10_callback)(uint16_t, void*);
  uint16_t (*val11_callback)(const void*);
  void (*set_val11_callback)(uint16_t, void*);
  uint16_t (*val12_callback)(const void*);
  void (*set_val12_callback)(uint16_t, void*);
  uint16_t (*val13_callback)(const void*);
  void (*set_val13_callback)(uint16_t, void*);
  uint16_t (*val14_callback)(const void*);
  void (*set_val14_callback)(uint16_t, void*);
  uint16_t (*val15_callback)(const void*);
  void (*set_val15_callback)(uint16_t, void*);
  uint16_t (*val16_callback)(const void*);
  void (*set_val16_callback)(uint16_t, void*);
  uint16_t (*val17_callback)(const void*);
  void (*set_val17_callback)(uint16_t, void*);
  uint16_t (*val18_callback)(const void*);
  void (*set_val18_callback)(uint16_t, void*);
  uint16_t (*val19_callback)(const void*);
  void (*set_val19_callback)(uint16_t, void*);
  uint16_t (*val20_callback)(const void*);
  void (*set_val20_callback)(uint16_t, void*);
  uint16_t (*val21_callback)(const void*);
  void (*set_val21_callback)(uint16_t, void*);
  uint16_t (*val22_callback)(const void*);
  void (*set_val22_callback)(uint16_t, void*);
  uint16_t (*val23_callback)(const void*);
  void (*set_val23_callback)(uint16_t, void*);
  uint16_t (*val24_callback)(const void*);
  void (*set_val24_callback)(uint16_t, void*);
  uint16_t (*val25_callback)(const void*);
  void (*set_val25_callback)(uint16_t, void*);
  uint16_t (*val26_callback)(const void*);
  void (*set_val26_callback)(uint16_t, void*);
  uint16_t (*val27_callback)(const void*);
  void (*set_val27_callback)(uint16_t, void*);
  uint16_t (*val28_callback)(const void*);
  void (*set_val28_callback)(uint16_t, void*);
  uint16_t (*val29_callback)(const void*);
  void (*set_val29_callback)(uint16_t, void*);
  uint16_t (*val30_callback)(const void*);
  void (*set_val30_callback)(uint16_t, void*);
  uint16_t (*val31_callback)(const void*);
  void (*set_val31_callback)(uint16_t, void*);
  uint16_t (*val32_callback)(const void*);
  void (*set_val32_callback)(uint16_t, void*);
  uint16_t (*val33_callback)(const void*);
  void (*set_val33_callback)(uint16_t, void*);
  uint16_t (*val34_callback)(const void*);
  void (*set_val34_callback)(uint16_t, void*);
  uint16_t (*val35_callback)(const void*);
  void (*set_val35_callback)(uint16_t, void*);
  uint16_t (*val36_callback)(const void*);
  void (*set_val36_callback)(uint16_t, void*);
  uint16_t (*val37_callback)(const void*);
  void (*set_val37_callback)(uint16_t, void*);
  uint16_t (*val38_callback)(const void*);
  void (*set_val38_callback)(uint16_t, void*);
  uint16_t (*val39_callback)(const void*);
  void (*set_val39_callback)(uint16_t, void*);
  uint16_t (*val40_callback)(const void*);
  void (*set_val40_callback)(uint16_t, void*);
  uint16_t (*val41_callback)(const void*);
  void (*set_val41_callback)(uint16_t, void*);
  uint16_t (*val42_callback)(const void*);
  void (*set_val42_callback)(uint16_t, void*);
  uint16_t (*val43_callback)(const void*);
  void (*set_val43_callback)(uint16_t, void*);
  uint16_t (*val44_callback)(const void*);
  void (*set_val44_callback)(uint16_t, void*);
  uint16_t (*val45_callback)(const void*);
  void (*set_val45_callback)(uint16_t, void*);
  uint16_t (*val46_callback)(const void*);
  void (*set_val46_callback)(uint16_t, void*);
  uint16_t (*val47_callback)(const void*);
  void (*set_val47_callback)(uint16_t, void*);
  uint16_t (*val48_callback)(const void*);
  void (*set_val48_callback)(uint16_t, void*);
  uint16_t (*val49_callback)(const void*);
  void (*set_val49_callback)(uint16_t, void*);
  uint16_t (*val50_callback)(const void*);
  void (*set_val50_callback)(uint16_t, void*);
  uint16_t (*val51_callback)(const void*);
  void (*set_val51_callback)(uint16_t, void*);
  uint16_t (*val52_callback)(const void*);
  void (*set_val52_callback)(uint16_t, void*);
  uint16_t (*val53_callback)(const void*);
  void (*set_val53_callback)(uint16_t, void*);
  uint16_t (*val54_callback)(const void*);
  void (*set_val54_callback)(uint16_t, void*);
  uint16_t (*val55_callback)(const void*);
  void (*set_val55_callback)(uint16_t, void*);
  uint16_t (*val56_callback)(const void*);
  void (*set_val56_callback)(uint16_t, void*);
  uint16_t (*val57_callback)(const void*);
  void (*set_val57_callback)(uint16_t, void*);
  uint16_t (*val58_callback)(const void*);
  void (*set_val58_callback)(uint16_t, void*);
  uint16_t (*val59_callback)(const void*);
  void (*set_val59_callback)(uint16_t, void*);
  uint16_t (*val60_callback)(const void*);
  void (*set_val60_callback)(uint16_t, void*);
  uint16_t (*val61_callback)(const void*);
  void (*set_val61_callback)(uint16_t, void*);
  uint16_t (*val62_callback)(const void*);
  void (*set_val62_callback)(uint16_t, void*);
  uint16_t (*val63_callback)(const void*);
  void (*set_val63_callback)(uint16_t, void*);
  uint16_t (*val64_callback)(const void*);
  void (*set_val64_callback)(uint16_t, void*);
  uint16_t (*val65_callback)(const void*);
  void (*set_val65_callback)(uint16_t, void*);
  uint16_t (*val66_callback)(const void*);
  void (*set_val66_callback)(uint16_t, void*);
  uint16_t (*val67_callback)(const void*);
  void (*set_val67_callback)(uint16_t, void*);
  uint16_t (*val68_callback)(const void*);
  void (*set_val68_callback)(uint16_t, void*);
  uint16_t (*val69_callback)(const void*);
  void (*set_val69_callback)(uint16_t, void*);
  uint16_t (*val70_callback)(const void*);
  void (*set_val70_callback)(uint16_t, void*);
  uint16_t (*val71_callback)(const void*);
  void (*set_val71_callback)(uint16_t, void*);
  uint16_t (*val72_callback)(const void*);
  void (*set_val72_callback)(uint16_t, void*);
  uint16_t (*val73_callback)(const void*);
  void (*set_val73_callback)(uint16_t, void*);
  uint16_t (*val74_callback)(const void*);
  void (*set_val74_callback)(uint16_t, void*);
  uint16_t (*val75_callback)(const void*);
  void (*set_val75_callback)(uint16_t, void*);
  uint16_t (*val76_callback)(const void*);
  void (*set_val76_callback)(uint16_t, void*);
  uint16_t (*val77_callback)(const void*);
  void (*set_val77_callback)(uint16_t, void*);
  uint16_t (*val78_callback)(const void*);
  void (*set_val78_callback)(uint16_t, void*);
  uint16_t (*val79_callback)(const void*);
  void (*set_val79_callback)(uint16_t, void*);
  uint16_t (*val80_callback)(const void*);
  void (*set_val80_callback)(uint16_t, void*);
  uint32_t (*timestamp_callback)(const void*);
  void (*set_timestamp_callback)(uint32_t, void*);
  uint16_t (*milliseconds_callback)(const void*);
  void (*set_milliseconds_callback)(uint16_t, void*);
  uint16_t (*sequence_callback)(const void*);
  void (*set_sequence_callback)(uint16_t, void*);
  uint16_t (*role_callback)(const void*);
  void (*set_role_callback)(uint16_t, void*);
  Alg (*algorithm_callback)(const void*);
  void (*set_algorithm_callback)(Alg, void*);
  uint16_t (*n_callback)(const void*);
  void (*set_n_callback)(uint16_t, void*);
} Model6CallbackAdapter;

typedef struct Model6StatefulAdapter {
  uint16_t x;
  uint16_t offset;
  uint16_t value1;
  uint16_t val2;
  uint16_t val3;
  uint16_t val4;
  uint16_t val5;
  uint16_t val6;
  uint16_t val7;
  uint16_t val8;
  uint16_t val9;
  uint16_t val10;
  uint16_t val11;
  uint16_t val12;
  uint16_t val13;
  uint16_t val14;
  uint16_t val15;
  uint16_t val16;
  uint16_t val17;
  uint16_t val18;
  uint16_t val19;
  uint16_t val20;
  uint16_t val21;
  uint16_t val22;
  uint16_t val23;
  uint16_t val24;
  uint16_t val25;
  uint16_t val26;
  uint16_t val27;
  uint16_t val28;
  uint16_t val29;
  uint16_t val30;
  uint16_t val31;
  uint16_t val32;
  uint16_t val33;
  uint16_t val34;
  uint16_t val35;
  uint16_t val36;
  uint16_t val37;
  uint16_t val38;
  uint16_t val39;
  uint16_t val40;
  uint16_t val41;
  uint16_t val42;
  uint16_t val43;
  uint16_t val44;
  uint16_t val45;
  uint16_t val46;
  uint16_t val47;
  uint16_t val48;
  uint16_t val49;
  uint16_t val50;
  uint16_t val51;
  uint16_t val52;
  uint16_t val53;
  uint16_t val54;
  uint16_t val55;
  uint16_t val56;
  uint16_t val57;
  uint16_t val58;
  uint16_t val59;
  uint16_t val60;
  uint16_t val61;
  uint16_t val62;
  uint16_t val63;
  uint16_t val64;
  uint16_t val65;
  uint16_t val66;
  uint16_t val67;
  uint16_t val68;
  uint16_t val69;
  uint16_t val70;
  uint16_t val71;
  uint16_t val72;
  uint16_t val73;
  uint16_t val74;
  uint16_t val75;
  uint16_t val76;
  uint16_t val77;
  uint16_t val78;
  uint16_t val79;
  uint16_t val80;
  uint32_t timestamp;
  uint16_t milliseconds;
  uint16_t sequence;
  uint16_t role;
  Alg algorithm;
  uint16_t n;
} Model6StatefulAdapter;

typedef struct Model7CallbackAdapter {
  void *context;
  uint16_t (*request_sequence_callback)(const void*);
  Sts (*status_callback)(const void*);
  uint32_t (*timestamp_callback)(const void*);
  uint16_t (*milliseconds_callback)(const void*);
  uint16_t (*sequence_callback)(const void*);
  Alm (*alarm_callback)(const void*);
  Alg (*algorithm_callback)(const void*);
  uint16_t (*n_callback)(const void*);
  void (*set_n_callback)(uint16_t, void*);
} Model7CallbackAdapter;

typedef struct Model7StatefulAdapter {
  uint16_t request_sequence;
  Sts status;
  uint32_t timestamp;
  uint16_t milliseconds;
  uint16_t sequence;
  Alm alarm;
  Alg algorithm;
  uint16_t n;
} Model7StatefulAdapter;

typedef struct Model8CallbackAdapter {
  void *context;
  Fmt (*format_callback)(const void*);
  uint16_t (*n_callback)(const void*);
} Model8CallbackAdapter;

typedef struct Model8StatefulAdapter {
  Fmt format;
  uint16_t n;
} Model8StatefulAdapter;

typedef struct Model10CallbackAdapter {
  void *context;
  St (*interface_status_callback)(const void*);
  uint16_t (*interface_control_callback)(const void*);
  void (*set_interface_control_callback)(uint16_t, void*);
  Typ (*physical_access_type_callback)(const void*);
} Model10CallbackAdapter;

typedef struct Model10StatefulAdapter {
  St interface_status;
  uint16_t interface_control;
  Typ physical_access_type;
} Model10StatefulAdapter;

typedef struct Model11CallbackAdapter {
  void *context;
  uint16_t (*ethernet_link_speed_callback)(const void*);
  uint16_t (*interface_status_flags_callback)(const void*);
  St (*link_state_callback)(const void*);
  const uint8_t *(*mac_callback)(const void*);
  const char *(*name_callback)(const void*);
  void (*set_name_callback)(const char*, void*);
  uint16_t (*control_callback)(const void*);
  void (*set_control_callback)(uint16_t, void*);
  uint16_t (*forced_speed_callback)(const void*);
  void (*set_forced_speed_callback)(uint16_t, void*);
} Model11CallbackAdapter;

typedef struct Model11StatefulAdapter {
  uint16_t ethernet_link_speed;
  uint16_t interface_status_flags;
  St link_state;
  uint8_t mac[6];
  char name[8];
  uint16_t control;
  uint16_t forced_speed;
} Model11StatefulAdapter;

typedef struct Model12CallbackAdapter {
  void *context;
  const char *(*name_callback)(const void*);
  void (*set_name_callback)(const char*, void*);
  CfgSt (*config_status_callback)(const void*);
  uint16_t (*change_status_callback)(const void*);
  uint16_t (*config_capability_callback)(const void*);
  Cfg (*i_pv4_config_callback)(const void*);
  void (*set_i_pv4_config_callback)(Cfg, void*);
  Ctl (*control_callback)(const void*);
  void (*set_control_callback)(Ctl, void*);
  const char *(*ip_callback)(const void*);
  void (*set_ip_callback)(const char*, void*);
  const char *(*netmask_callback)(const void*);
  void (*set_netmask_callback)(const char*, void*);
  const char *(*gateway_callback)(const void*);
  void (*set_gateway_callback)(const char*, void*);
  const char *(*dns1_callback)(const void*);
  void (*set_dns1_callback)(const char*, void*);
  const char *(*dns2_callback)(const void*);
  void (*set_dns2_callback)(const char*, void*);
  const char *(*ntp1_callback)(const void*);
  void (*set_ntp1_callback)(const char*, void*);
  const char *(*ntp2_callback)(const void*);
  void (*set_ntp2_callback)(const char*, void*);
  const char *(*domain_callback)(const void*);
  void (*set_domain_callback)(const char*, void*);
  const char *(*host_name_callback)(const void*);
  void (*set_host_name_callback)(const char*, void*);
} Model12CallbackAdapter;

typedef struct Model12StatefulAdapter {
  char name[8];
  CfgSt config_status;
  uint16_t change_status;
  uint16_t config_capability;
  Cfg i_pv4_config;
  Ctl control;
  char ip[16];
  char netmask[16];
  char gateway[16];
  char dns1[16];
  char dns2[16];
  char ntp1[24];
  char ntp2[24];
  char domain[24];
  char host_name[24];
} Model12StatefulAdapter;

typedef struct Model13CallbackAdapter {
  void *context;
  const char *(*name_callback)(const void*);
  void (*set_name_callback)(const char*, void*);
  CfgSt (*config_status_callback)(const void*);
  uint16_t (*change_status_callback)(const void*);
  uint16_t (*config_capability_callback)(const void*);
  Cfg (*i_pv6_config_callback)(const void*);
  void (*set_i_pv6_config_callback)(Cfg, void*);
  Ctl (*control_callback)(const void*);
  void (*set_control_callback)(Ctl, void*);
  const char *(*ip_callback)(const void*);
  void (*set_ip_callback)(const char*, void*);
  const char *(*cidr_callback)(const void*);
  void (*set_cidr_callback)(const char*, void*);
  const char *(*gateway_callback)(const void*);
  void (*set_gateway_callback)(const char*, void*);
  const char *(*dns1_callback)(const void*);
  void (*set_dns1_callback)(const char*, void*);
  const char *(*dns2_callback)(const void*);
  void (*set_dns2_callback)(const char*, void*);
  const char *(*ntp1_callback)(const void*);
  void (*set_ntp1_callback)(const char*, void*);
  const char *(*ntp2_callback)(const void*);
  void (*set_ntp2_callback)(const char*, void*);
  const char *(*domain_callback)(const void*);
  void (*set_domain_callback)(const char*, void*);
  const char *(*host_name_callback)(const void*);
  void (*set_host_name_callback)(const char*, void*);
} Model13CallbackAdapter;

typedef struct Model13StatefulAdapter {
  char name[8];
  CfgSt config_status;
  uint16_t change_status;
  uint16_t config_capability;
  Cfg i_pv6_config;
  Ctl control;
  char ip[40];
  char cidr[40];
  char gateway[40];
  char dns1[40];
  char dns2[40];
  char ntp1[40];
  char ntp2[40];
  char domain[24];
  char host_name[24];
} Model13StatefulAdapter;

typedef struct Model15CallbackAdapter {
  void *context;
  uint16_t (*clear_callback)(const void*);
  void (*set_clear_callback)(uint16_t, void*);
  uint32_t (*input_count_callback)(const void*);
  uint32_t (*input_unicast_count_callback)(const void*);
  uint32_t (*input_non_unicast_count_callback)(const void*);
  uint32_t (*input_discarded_count_callback)(const void*);
  uint32_t (*input_error_count_callback)(const void*);
  uint32_t (*input_unknown_count_callback)(const void*);
  uint32_t (*output_count_callback)(const void*);
  uint32_t (*output_unicast_count_callback)(const void*);
  uint32_t (*output_non_unicast_count_callback)(const void*);
  uint32_t (*output_discarded_count_callback)(const void*);
  uint32_t (*output_error_count_callback)(const void*);
} Model15CallbackAdapter;

typedef struct Model15StatefulAdapter {
  uint16_t clear;
  uint32_t input_count;
  uint32_t input_unicast_count;
  uint32_t input_non_unicast_count;
  uint32_t input_discarded_count;
  uint32_t input_error_count;
  uint32_t input_unknown_count;
  uint32_t output_count;
  uint32_t output_unicast_count;
  uint32_t output_non_unicast_count;
  uint32_t output_discarded_count;
  uint32_t output_error_count;
} Model15StatefulAdapter;

typedef struct Model16CallbackAdapter {
  void *context;
  const char *(*name_callback)(const void*);
  void (*set_name_callback)(const char*, void*);
  Cfg (*config_callback)(const void*);
  uint16_t (*control_callback)(const void*);
  void (*set_control_callback)(uint16_t, void*);
  const char *(*address_callback)(const void*);
  void (*set_address_callback)(const char*, void*);
  const char *(*netmask_callback)(const void*);
  void (*set_netmask_callback)(const char*, void*);
  const char *(*gateway_callback)(const void*);
  void (*set_gateway_callback)(const char*, void*);
  const char *(*dns1_callback)(const void*);
  void (*set_dns1_callback)(const char*, void*);
  const char *(*dns2_callback)(const void*);
  void (*set_dns2_callback)(const char*, void*);
  const uint8_t *(*mac_callback)(const void*);
  uint16_t (*link_control_callback)(const void*);
  void (*set_link_control_callback)(uint16_t, void*);
} Model16CallbackAdapter;

typedef struct Model16StatefulAdapter {
  char name[8];
  Cfg config;
  uint16_t control;
  char address[16];
  char netmask[16];
  char gateway[16];
  char dns1[16];
  char dns2[16];
  uint8_t mac[6];
  uint16_t link_control;
} Model16StatefulAdapter;

typedef struct Model17CallbackAdapter {
  void *context;
  const char *(*name_callback)(const void*);
  void (*set_name_callback)(const char*, void*);
  uint32_t (*rate_callback)(const void*);
  void (*set_rate_callback)(uint32_t, void*);
  uint16_t (*bits_callback)(const void*);
  void (*set_bits_callback)(uint16_t, void*);
  Pty (*parity_callback)(const void*);
  void (*set_parity_callback)(Pty, void*);
  Dup (*duplex_callback)(const void*);
  void (*set_duplex_callback)(Dup, void*);
  Flw (*flow_control_callback)(const void*);
  void (*set_flow_control_callback)(Flw, void*);
  Typ (*interface_type_callback)(const void*);
  Pcol (*protocol_callback)(const void*);
} Model17CallbackAdapter;

typedef struct Model17StatefulAdapter {
  char name[8];
  uint32_t rate;
  uint16_t bits;
  Pty parity;
  Dup duplex;
  Flw flow_control;
  Typ interface_type;
  Pcol protocol;
} Model17StatefulAdapter;

typedef struct Model18CallbackAdapter {
  void *context;
  const char *(*name_callback)(const void*);
  void (*set_name_callback)(const char*, void*);
  uint32_t (*imei_callback)(const void*);
  void (*set_imei_callback)(uint32_t, void*);
  const char *(*apn_callback)(const void*);
  void (*set_apn_callback)(const char*, void*);
  const char *(*number_callback)(const void*);
  void (*set_number_callback)(const char*, void*);
  const char *(*pin_callback)(const void*);
  void (*set_pin_callback)(const char*, void*);
} Model18CallbackAdapter;

typedef struct Model18StatefulAdapter {
  char name[8];
  uint32_t imei;
  char apn[8];
  char number[12];
  char pin[12];
} Model18StatefulAdapter;

typedef struct Model19CallbackAdapter {
  void *context;
  const char *(*name_callback)(const void*);
  void (*set_name_callback)(const char*, void*);
  uint32_t (*rate_callback)(const void*);
  void (*set_rate_callback)(uint32_t, void*);
  uint16_t (*bits_callback)(const void*);
  void (*set_bits_callback)(uint16_t, void*);
  Pty (*parity_callback)(const void*);
  void (*set_parity_callback)(Pty, void*);
  Dup (*duplex_callback)(const void*);
  void (*set_duplex_callback)(Dup, void*);
  Flw (*flow_control_callback)(const void*);
  void (*set_flow_control_callback)(Flw, void*);
  Auth (*authentication_callback)(const void*);
  const char *(*username_callback)(const void*);
  const char *(*password_callback)(const void*);
} Model19CallbackAdapter;

typedef struct Model19StatefulAdapter {
  char name[8];
  uint32_t rate;
  uint16_t bits;
  Pty parity;
  Dup duplex;
  Flw flow_control;
  Auth authentication;
  char username[24];
  char password[12];
} Model19StatefulAdapter;

typedef struct Model101CallbackAdapter {
  void *context;
  uint16_t (*amps_callback)(const void*);
  uint16_t (*amps_phase_a_callback)(const void*);
  uint16_t (*amps_phase_b_callback)(const void*);
  uint16_t (*amps_phase_c_callback)(const void*);
  uint16_t (*a_sf_callback)(const void*);
  uint16_t (*phase_voltage_ab_callback)(const void*);
  uint16_t (*phase_voltage_bc_callback)(const void*);
  uint16_t (*phase_voltage_ca_callback)(const void*);
  uint16_t (*phase_voltage_an_callback)(const void*);
  uint16_t (*phase_voltage_bn_callback)(const void*);
  uint16_t (*phase_voltage_cn_callback)(const void*);
  uint16_t (*v_sf_callback)(const void*);
  int16_t (*watts_callback)(const void*);
  uint16_t (*w_sf_callback)(const void*);
  uint16_t (*hz_callback)(const void*);
  uint16_t (*hz_sf_callback)(const void*);
  int16_t (*va_callback)(const void*);
  uint16_t (*va_sf_callback)(const void*);
  int16_t (*v_ar_callback)(const void*);
  uint16_t (*v_ar_sf_callback)(const void*);
  int16_t (*pf_callback)(const void*);
  uint16_t (*pf_sf_callback)(const void*);
  uint32_t (*watt_hours_callback)(const void*);
  uint16_t (*wh_sf_callback)(const void*);
  uint16_t (*dc_amps_callback)(const void*);
  uint16_t (*dca_sf_callback)(const void*);
  uint16_t (*dc_voltage_callback)(const void*);
  uint16_t (*dcv_sf_callback)(const void*);
  int16_t (*dc_watts_callback)(const void*);
  uint16_t (*dcw_sf_callback)(const void*);
  int16_t (*cabinet_temperature_callback)(const void*);
  int16_t (*heat_sink_temperature_callback)(const void*);
  int16_t (*transformer_temperature_callback)(const void*);
  int16_t (*other_temperature_callback)(const void*);
  uint16_t (*tmp_sf_callback)(const void*);
  St (*operating_state_callback)(const void*);
  uint16_t (*vendor_operating_state_callback)(const void*);
  uint32_t (*event1_callback)(const void*);
  uint32_t (*event_bitfield_2_callback)(const void*);
  uint32_t (*vendor_event_bitfield_1_callback)(const void*);
  uint32_t (*vendor_event_bitfield_2_callback)(const void*);
  uint32_t (*vendor_event_bitfield_3_callback)(const void*);
  uint32_t (*vendor_event_bitfield_4_callback)(const void*);
} Model101CallbackAdapter;

typedef struct Model101StatefulAdapter {
  uint16_t amps;
  uint16_t amps_phase_a;
  uint16_t amps_phase_b;
  uint16_t amps_phase_c;
  uint16_t a_sf;
  uint16_t phase_voltage_ab;
  uint16_t phase_voltage_bc;
  uint16_t phase_voltage_ca;
  uint16_t phase_voltage_an;
  uint16_t phase_voltage_bn;
  uint16_t phase_voltage_cn;
  uint16_t v_sf;
  int16_t watts;
  uint16_t w_sf;
  uint16_t hz;
  uint16_t hz_sf;
  int16_t va;
  uint16_t va_sf;
  int16_t v_ar;
  uint16_t v_ar_sf;
  int16_t pf;
  uint16_t pf_sf;
  uint32_t watt_hours;
  uint16_t wh_sf;
  uint16_t dc_amps;
  uint16_t dca_sf;
  uint16_t dc_voltage;
  uint16_t dcv_sf;
  int16_t dc_watts;
  uint16_t dcw_sf;
  int16_t cabinet_temperature;
  int16_t heat_sink_temperature;
  int16_t transformer_temperature;
  int16_t other_temperature;
  uint16_t tmp_sf;
  St operating_state;
  uint16_t vendor_operating_state;
  uint32_t event1;
  uint32_t event_bitfield_2;
  uint32_t vendor_event_bitfield_1;
  uint32_t vendor_event_bitfield_2;
  uint32_t vendor_event_bitfield_3;
  uint32_t vendor_event_bitfield_4;
} Model101StatefulAdapter;

typedef struct Model102CallbackAdapter {
  void *context;
  uint16_t (*amps_callback)(const void*);
  uint16_t (*amps_phase_a_callback)(const void*);
  uint16_t (*amps_phase_b_callback)(const void*);
  uint16_t (*amps_phase_c_callback)(const void*);
  uint16_t (*a_sf_callback)(const void*);
  uint16_t (*phase_voltage_ab_callback)(const void*);
  uint16_t (*phase_voltage_bc_callback)(const void*);
  uint16_t (*phase_voltage_ca_callback)(const void*);
  uint16_t (*phase_voltage_an_callback)(const void*);
  uint16_t (*phase_voltage_bn_callback)(const void*);
  uint16_t (*phase_voltage_cn_callback)(const void*);
  uint16_t (*v_sf_callback)(const void*);
  int16_t (*watts_callback)(const void*);
  uint16_t (*w_sf_callback)(const void*);
  uint16_t (*hz_callback)(const void*);
  uint16_t (*hz_sf_callback)(const void*);
  int16_t (*va_callback)(const void*);
  uint16_t (*va_sf_callback)(const void*);
  int16_t (*v_ar_callback)(const void*);
  uint16_t (*v_ar_sf_callback)(const void*);
  int16_t (*pf_callback)(const void*);
  uint16_t (*pf_sf_callback)(const void*);
  uint32_t (*watt_hours_callback)(const void*);
  uint16_t (*wh_sf_callback)(const void*);
  uint16_t (*dc_amps_callback)(const void*);
  uint16_t (*dca_sf_callback)(const void*);
  uint16_t (*dc_voltage_callback)(const void*);
  uint16_t (*dcv_sf_callback)(const void*);
  int16_t (*dc_watts_callback)(const void*);
  uint16_t (*dcw_sf_callback)(const void*);
  int16_t (*cabinet_temperature_callback)(const void*);
  int16_t (*heat_sink_temperature_callback)(const void*);
  int16_t (*transformer_temperature_callback)(const void*);
  int16_t (*other_temperature_callback)(const void*);
  uint16_t (*tmp_sf_callback)(const void*);
  St (*operating_state_callback)(const void*);
  uint16_t (*vendor_operating_state_callback)(const void*);
  uint32_t (*event1_callback)(const void*);
  uint32_t (*event_bitfield_2_callback)(const void*);
  uint32_t (*vendor_event_bitfield_1_callback)(const void*);
  uint32_t (*vendor_event_bitfield_2_callback)(const void*);
  uint32_t (*vendor_event_bitfield_3_callback)(const void*);
  uint32_t (*vendor_event_bitfield_4_callback)(const void*);
} Model102CallbackAdapter;

typedef struct Model102StatefulAdapter {
  uint16_t amps;
  uint16_t amps_phase_a;
  uint16_t amps_phase_b;
  uint16_t amps_phase_c;
  uint16_t a_sf;
  uint16_t phase_voltage_ab;
  uint16_t phase_voltage_bc;
  uint16_t phase_voltage_ca;
  uint16_t phase_voltage_an;
  uint16_t phase_voltage_bn;
  uint16_t phase_voltage_cn;
  uint16_t v_sf;
  int16_t watts;
  uint16_t w_sf;
  uint16_t hz;
  uint16_t hz_sf;
  int16_t va;
  uint16_t va_sf;
  int16_t v_ar;
  uint16_t v_ar_sf;
  int16_t pf;
  uint16_t pf_sf;
  uint32_t watt_hours;
  uint16_t wh_sf;
  uint16_t dc_amps;
  uint16_t dca_sf;
  uint16_t dc_voltage;
  uint16_t dcv_sf;
  int16_t dc_watts;
  uint16_t dcw_sf;
  int16_t cabinet_temperature;
  int16_t heat_sink_temperature;
  int16_t transformer_temperature;
  int16_t other_temperature;
  uint16_t tmp_sf;
  St operating_state;
  uint16_t vendor_operating_state;
  uint32_t event1;
  uint32_t event_bitfield_2;
  uint32_t vendor_event_bitfield_1;
  uint32_t vendor_event_bitfield_2;
  uint32_t vendor_event_bitfield_3;
  uint32_t vendor_event_bitfield_4;
} Model102StatefulAdapter;

typedef struct Model103CallbackAdapter {
  void *context;
  uint16_t (*amps_callback)(const void*);
  uint16_t (*amps_phase_a_callback)(const void*);
  uint16_t (*amps_phase_b_callback)(const void*);
  uint16_t (*amps_phase_c_callback)(const void*);
  uint16_t (*a_sf_callback)(const void*);
  uint16_t (*phase_voltage_ab_callback)(const void*);
  uint16_t (*phase_voltage_bc_callback)(const void*);
  uint16_t (*phase_voltage_ca_callback)(const void*);
  uint16_t (*phase_voltage_an_callback)(const void*);
  uint16_t (*phase_voltage_bn_callback)(const void*);
  uint16_t (*phase_voltage_cn_callback)(const void*);
  uint16_t (*v_sf_callback)(const void*);
  int16_t (*watts_callback)(const void*);
  uint16_t (*w_sf_callback)(const void*);
  uint16_t (*hz_callback)(const void*);
  uint16_t (*hz_sf_callback)(const void*);
  int16_t (*va_callback)(const void*);
  uint16_t (*va_sf_callback)(const void*);
  int16_t (*v_ar_callback)(const void*);
  uint16_t (*v_ar_sf_callback)(const void*);
  int16_t (*pf_callback)(const void*);
  uint16_t (*pf_sf_callback)(const void*);
  uint32_t (*watt_hours_callback)(const void*);
  uint16_t (*wh_sf_callback)(const void*);
  uint16_t (*dc_amps_callback)(const void*);
  uint16_t (*dca_sf_callback)(const void*);
  uint16_t (*dc_voltage_callback)(const void*);
  uint16_t (*dcv_sf_callback)(const void*);
  int16_t (*dc_watts_callback)(const void*);
  uint16_t (*dcw_sf_callback)(const void*);
  int16_t (*cabinet_temperature_callback)(const void*);
  int16_t (*heat_sink_temperature_callback)(const void*);
  int16_t (*transformer_temperature_callback)(const void*);
  int16_t (*other_temperature_callback)(const void*);
  uint16_t (*tmp_sf_callback)(const void*);
  St (*operating_state_callback)(const void*);
  uint16_t (*vendor_operating_state_callback)(const void*);
  uint32_t (*event1_callback)(const void*);
  uint32_t (*event_bitfield_2_callback)(const void*);
  uint32_t (*vendor_event_bitfield_1_callback)(const void*);
  uint32_t (*vendor_event_bitfield_2_callback)(const void*);
  uint32_t (*vendor_event_bitfield_3_callback)(const void*);
  uint32_t (*vendor_event_bitfield_4_callback)(const void*);
} Model103CallbackAdapter;

typedef struct Model103StatefulAdapter {
  uint16_t amps;
  uint16_t amps_phase_a;
  uint16_t amps_phase_b;
  uint16_t amps_phase_c;
  uint16_t a_sf;
  uint16_t phase_voltage_ab;
  uint16_t phase_voltage_bc;
  uint16_t phase_voltage_ca;
  uint16_t phase_voltage_an;
  uint16_t phase_voltage_bn;
  uint16_t phase_voltage_cn;
  uint16_t v_sf;
  int16_t watts;
  uint16_t w_sf;
  uint16_t hz;
  uint16_t hz_sf;
  int16_t va;
  uint16_t va_sf;
  int16_t v_ar;
  uint16_t v_ar_sf;
  int16_t pf;
  uint16_t pf_sf;
  uint32_t watt_hours;
  uint16_t wh_sf;
  uint16_t dc_amps;
  uint16_t dca_sf;
  uint16_t dc_voltage;
  uint16_t dcv_sf;
  int16_t dc_watts;
  uint16_t dcw_sf;
  int16_t cabinet_temperature;
  int16_t heat_sink_temperature;
  int16_t transformer_temperature;
  int16_t other_temperature;
  uint16_t tmp_sf;
  St operating_state;
  uint16_t vendor_operating_state;
  uint32_t event1;
  uint32_t event_bitfield_2;
  uint32_t vendor_event_bitfield_1;
  uint32_t vendor_event_bitfield_2;
  uint32_t vendor_event_bitfield_3;
  uint32_t vendor_event_bitfield_4;
} Model103StatefulAdapter;

typedef struct Model111CallbackAdapter {
  void *context;
  float (*amps_callback)(const void*);
  float (*amps_phase_a_callback)(const void*);
  float (*amps_phase_b_callback)(const void*);
  float (*amps_phase_c_callback)(const void*);
  float (*phase_voltage_ab_callback)(const void*);
  float (*phase_voltage_bc_callback)(const void*);
  float (*phase_voltage_ca_callback)(const void*);
  float (*phase_voltage_an_callback)(const void*);
  float (*phase_voltage_bn_callback)(const void*);
  float (*phase_voltage_cn_callback)(const void*);
  float (*watts_callback)(const void*);
  float (*hz_callback)(const void*);
  float (*va_callback)(const void*);
  float (*v_ar_callback)(const void*);
  float (*pf_callback)(const void*);
  float (*watt_hours_callback)(const void*);
  float (*dc_amps_callback)(const void*);
  float (*dc_voltage_callback)(const void*);
  float (*dc_watts_callback)(const void*);
  float (*cabinet_temperature_callback)(const void*);
  float (*heat_sink_temperature_callback)(const void*);
  float (*transformer_temperature_callback)(const void*);
  float (*other_temperature_callback)(const void*);
  St (*operating_state_callback)(const void*);
  uint16_t (*vendor_operating_state_callback)(const void*);
  uint32_t (*event1_callback)(const void*);
  uint32_t (*event_bitfield_2_callback)(const void*);
  uint32_t (*vendor_event_bitfield_1_callback)(const void*);
  uint32_t (*vendor_event_bitfield_2_callback)(const void*);
  uint32_t (*vendor_event_bitfield_3_callback)(const void*);
  uint32_t (*vendor_event_bitfield_4_callback)(const void*);
} Model111CallbackAdapter;

typedef struct Model111StatefulAdapter {
  float amps;
  float amps_phase_a;
  float amps_phase_b;
  float amps_phase_c;
  float phase_voltage_ab;
  float phase_voltage_bc;
  float phase_voltage_ca;
  float phase_voltage_an;
  float phase_voltage_bn;
  float phase_voltage_cn;
  float watts;
  float hz;
  float va;
  float v_ar;
  float pf;
  float watt_hours;
  float dc_amps;
  float dc_voltage;
  float dc_watts;
  float cabinet_temperature;
  float heat_sink_temperature;
  float transformer_temperature;
  float other_temperature;
  St operating_state;
  uint16_t vendor_operating_state;
  uint32_t event1;
  uint32_t event_bitfield_2;
  uint32_t vendor_event_bitfield_1;
  uint32_t vendor_event_bitfield_2;
  uint32_t vendor_event_bitfield_3;
  uint32_t vendor_event_bitfield_4;
} Model111StatefulAdapter;

typedef struct Model112CallbackAdapter {
  void *context;
  float (*amps_callback)(const void*);
  float (*amps_phase_a_callback)(const void*);
  float (*amps_phase_b_callback)(const void*);
  float (*amps_phase_c_callback)(const void*);
  float (*phase_voltage_ab_callback)(const void*);
  float (*phase_voltage_bc_callback)(const void*);
  float (*phase_voltage_ca_callback)(const void*);
  float (*phase_voltage_an_callback)(const void*);
  float (*phase_voltage_bn_callback)(const void*);
  float (*phase_voltage_cn_callback)(const void*);
  float (*watts_callback)(const void*);
  float (*hz_callback)(const void*);
  float (*va_callback)(const void*);
  float (*v_ar_callback)(const void*);
  float (*pf_callback)(const void*);
  float (*watt_hours_callback)(const void*);
  float (*dc_amps_callback)(const void*);
  float (*dc_voltage_callback)(const void*);
  float (*dc_watts_callback)(const void*);
  float (*cabinet_temperature_callback)(const void*);
  float (*heat_sink_temperature_callback)(const void*);
  float (*transformer_temperature_callback)(const void*);
  float (*other_temperature_callback)(const void*);
  St (*operating_state_callback)(const void*);
  uint16_t (*vendor_operating_state_callback)(const void*);
  uint32_t (*event1_callback)(const void*);
  uint32_t (*event_bitfield_2_callback)(const void*);
  uint32_t (*vendor_event_bitfield_1_callback)(const void*);
  uint32_t (*vendor_event_bitfield_2_callback)(const void*);
  uint32_t (*vendor_event_bitfield_3_callback)(const void*);
  uint32_t (*vendor_event_bitfield_4_callback)(const void*);
} Model112CallbackAdapter;

typedef struct Model112StatefulAdapter {
  float amps;
  float amps_phase_a;
  float amps_phase_b;
  float amps_phase_c;
  float phase_voltage_ab;
  float phase_voltage_bc;
  float phase_voltage_ca;
  float phase_voltage_an;
  float phase_voltage_bn;
  float phase_voltage_cn;
  float watts;
  float hz;
  float va;
  float v_ar;
  float pf;
  float watt_hours;
  float dc_amps;
  float dc_voltage;
  float dc_watts;
  float cabinet_temperature;
  float heat_sink_temperature;
  float transformer_temperature;
  float other_temperature;
  St operating_state;
  uint16_t vendor_operating_state;
  uint32_t event1;
  uint32_t event_bitfield_2;
  uint32_t vendor_event_bitfield_1;
  uint32_t vendor_event_bitfield_2;
  uint32_t vendor_event_bitfield_3;
  uint32_t vendor_event_bitfield_4;
} Model112StatefulAdapter;

typedef struct Model113CallbackAdapter {
  void *context;
  float (*amps_callback)(const void*);
  float (*amps_phase_a_callback)(const void*);
  float (*amps_phase_b_callback)(const void*);
  float (*amps_phase_c_callback)(const void*);
  float (*phase_voltage_ab_callback)(const void*);
  float (*phase_voltage_bc_callback)(const void*);
  float (*phase_voltage_ca_callback)(const void*);
  float (*phase_voltage_an_callback)(const void*);
  float (*phase_voltage_bn_callback)(const void*);
  float (*phase_voltage_cn_callback)(const void*);
  float (*watts_callback)(const void*);
  float (*hz_callback)(const void*);
  float (*va_callback)(const void*);
  float (*v_ar_callback)(const void*);
  float (*pf_callback)(const void*);
  float (*watt_hours_callback)(const void*);
  float (*dc_amps_callback)(const void*);
  float (*dc_voltage_callback)(const void*);
  float (*dc_watts_callback)(const void*);
  float (*cabinet_temperature_callback)(const void*);
  float (*heat_sink_temperature_callback)(const void*);
  float (*transformer_temperature_callback)(const void*);
  float (*other_temperature_callback)(const void*);
  St (*operating_state_callback)(const void*);
  uint16_t (*vendor_operating_state_callback)(const void*);
  uint32_t (*event1_callback)(const void*);
  uint32_t (*event_bitfield_2_callback)(const void*);
  uint32_t (*vendor_event_bitfield_1_callback)(const void*);
  uint32_t (*vendor_event_bitfield_2_callback)(const void*);
  uint32_t (*vendor_event_bitfield_3_callback)(const void*);
  uint32_t (*vendor_event_bitfield_4_callback)(const void*);
} Model113CallbackAdapter;

typedef struct Model113StatefulAdapter {
  float amps;
  float amps_phase_a;
  float amps_phase_b;
  float amps_phase_c;
  float phase_voltage_ab;
  float phase_voltage_bc;
  float phase_voltage_ca;
  float phase_voltage_an;
  float phase_voltage_bn;
  float phase_voltage_cn;
  float watts;
  float hz;
  float va;
  float v_ar;
  float pf;
  float watt_hours;
  float dc_amps;
  float dc_voltage;
  float dc_watts;
  float cabinet_temperature;
  float heat_sink_temperature;
  float transformer_temperature;
  float other_temperature;
  St operating_state;
  uint16_t vendor_operating_state;
  uint32_t event1;
  uint32_t event_bitfield_2;
  uint32_t vendor_event_bitfield_1;
  uint32_t vendor_event_bitfield_2;
  uint32_t vendor_event_bitfield_3;
  uint32_t vendor_event_bitfield_4;
} Model113StatefulAdapter;

typedef struct Model120CallbackAdapter {
  void *context;
  DerTyp (*der_typ_callback)(const void*);
  uint16_t (*w_rtg_callback)(const void*);
  uint16_t (*w_rtg_sf_callback)(const void*);
  uint16_t (*va_rtg_callback)(const void*);
  uint16_t (*va_rtg_sf_callback)(const void*);
  int16_t (*v_ar_rtg_q1_callback)(const void*);
  int16_t (*v_ar_rtg_q2_callback)(const void*);
  int16_t (*v_ar_rtg_q3_callback)(const void*);
  int16_t (*v_ar_rtg_q4_callback)(const void*);
  uint16_t (*v_ar_rtg_sf_callback)(const void*);
  uint16_t (*a_rtg_callback)(const void*);
  uint16_t (*a_rtg_sf_callback)(const void*);
  int16_t (*pf_rtg_q1_callback)(const void*);
  int16_t (*pf_rtg_q2_callback)(const void*);
  int16_t (*pf_rtg_q3_callback)(const void*);
  int16_t (*pf_rtg_q4_callback)(const void*);
  uint16_t (*pf_rtg_sf_callback)(const void*);
  uint16_t (*wh_rtg_callback)(const void*);
  uint16_t (*wh_rtg_sf_callback)(const void*);
  uint16_t (*ahr_rtg_callback)(const void*);
  uint16_t (*ahr_rtg_sf_callback)(const void*);
  uint16_t (*max_cha_rte_callback)(const void*);
  uint16_t (*max_cha_rte_sf_callback)(const void*);
  uint16_t (*max_dis_cha_rte_callback)(const void*);
  uint16_t (*max_dis_cha_rte_sf_callback)(const void*);
} Model120CallbackAdapter;

typedef struct Model120StatefulAdapter {
  DerTyp der_typ;
  uint16_t w_rtg;
  uint16_t w_rtg_sf;
  uint16_t va_rtg;
  uint16_t va_rtg_sf;
  int16_t v_ar_rtg_q1;
  int16_t v_ar_rtg_q2;
  int16_t v_ar_rtg_q3;
  int16_t v_ar_rtg_q4;
  uint16_t v_ar_rtg_sf;
  uint16_t a_rtg;
  uint16_t a_rtg_sf;
  int16_t pf_rtg_q1;
  int16_t pf_rtg_q2;
  int16_t pf_rtg_q3;
  int16_t pf_rtg_q4;
  uint16_t pf_rtg_sf;
  uint16_t wh_rtg;
  uint16_t wh_rtg_sf;
  uint16_t ahr_rtg;
  uint16_t ahr_rtg_sf;
  uint16_t max_cha_rte;
  uint16_t max_cha_rte_sf;
  uint16_t max_dis_cha_rte;
  uint16_t max_dis_cha_rte_sf;
} Model120StatefulAdapter;

typedef struct Model121CallbackAdapter {
  void *context;
  uint16_t (*w_max_callback)(const void*);
  void (*set_w_max_callback)(uint16_t, void*);
  uint16_t (*v_ref_callback)(const void*);
  void (*set_v_ref_callback)(uint16_t, void*);
  int16_t (*v_ref_ofs_callback)(const void*);
  void (*set_v_ref_ofs_callback)(int16_t, void*);
  uint16_t (*v_max_callback)(const void*);
  void (*set_v_max_callback)(uint16_t, void*);
  uint16_t (*v_min_callback)(const void*);
  void (*set_v_min_callback)(uint16_t, void*);
  uint16_t (*va_max_callback)(const void*);
  void (*set_va_max_callback)(uint16_t, void*);
  int16_t (*v_ar_max_q1_callback)(const void*);
  void (*set_v_ar_max_q1_callback)(int16_t, void*);
  int16_t (*v_ar_max_q2_callback)(const void*);
  void (*set_v_ar_max_q2_callback)(int16_t, void*);
  int16_t (*v_ar_max_q3_callback)(const void*);
  void (*set_v_ar_max_q3_callback)(int16_t, void*);
  int16_t (*v_ar_max_q4_callback)(const void*);
  void (*set_v_ar_max_q4_callback)(int16_t, void*);
  uint16_t (*w_gra_callback)(const void*);
  void (*set_w_gra_callback)(uint16_t, void*);
  int16_t (*pf_min_q1_callback)(const void*);
  void (*set_pf_min_q1_callback)(int16_t, void*);
  int16_t (*pf_min_q2_callback)(const void*);
  void (*set_pf_min_q2_callback)(int16_t, void*);
  int16_t (*pf_min_q3_callback)(const void*);
  void (*set_pf_min_q3_callback)(int16_t, void*);
  int16_t (*pf_min_q4_callback)(const void*);
  void (*set_pf_min_q4_callback)(int16_t, void*);
  VArAct (*v_ar_act_callback)(const void*);
  void (*set_v_ar_act_callback)(VArAct, void*);
  ClcTotVa (*clc_tot_va_callback)(const void*);
  void (*set_clc_tot_va_callback)(ClcTotVa, void*);
  uint16_t (*max_rmp_rte_callback)(const void*);
  void (*set_max_rmp_rte_callback)(uint16_t, void*);
  uint16_t (*ecp_nom_hz_callback)(const void*);
  void (*set_ecp_nom_hz_callback)(uint16_t, void*);
  ConnPh (*conn_ph_callback)(const void*);
  void (*set_conn_ph_callback)(ConnPh, void*);
  uint16_t (*w_max_sf_callback)(const void*);
  uint16_t (*v_ref_sf_callback)(const void*);
  uint16_t (*v_ref_ofs_sf_callback)(const void*);
  uint16_t (*v_min_max_sf_callback)(const void*);
  uint16_t (*va_max_sf_callback)(const void*);
  uint16_t (*v_ar_max_sf_callback)(const void*);
  uint16_t (*w_gra_sf_callback)(const void*);
  uint16_t (*pf_min_sf_callback)(const void*);
  uint16_t (*max_rmp_rte_sf_callback)(const void*);
  uint16_t (*ecp_nom_hz_sf_callback)(const void*);
} Model121CallbackAdapter;

typedef struct Model121StatefulAdapter {
  uint16_t w_max;
  uint16_t v_ref;
  int16_t v_ref_ofs;
  uint16_t v_max;
  uint16_t v_min;
  uint16_t va_max;
  int16_t v_ar_max_q1;
  int16_t v_ar_max_q2;
  int16_t v_ar_max_q3;
  int16_t v_ar_max_q4;
  uint16_t w_gra;
  int16_t pf_min_q1;
  int16_t pf_min_q2;
  int16_t pf_min_q3;
  int16_t pf_min_q4;
  VArAct v_ar_act;
  ClcTotVa clc_tot_va;
  uint16_t max_rmp_rte;
  uint16_t ecp_nom_hz;
  ConnPh conn_ph;
  uint16_t w_max_sf;
  uint16_t v_ref_sf;
  uint16_t v_ref_ofs_sf;
  uint16_t v_min_max_sf;
  uint16_t va_max_sf;
  uint16_t v_ar_max_sf;
  uint16_t w_gra_sf;
  uint16_t pf_min_sf;
  uint16_t max_rmp_rte_sf;
  uint16_t ecp_nom_hz_sf;
} Model121StatefulAdapter;

typedef struct Model122CallbackAdapter {
  void *context;
  uint16_t (*pv_conn_callback)(const void*);
  uint16_t (*stor_conn_callback)(const void*);
  uint16_t (*ecp_conn_callback)(const void*);
  uint64_t (*act_wh_callback)(const void*);
  uint64_t (*act_v_ah_callback)(const void*);
  uint64_t (*act_v_arh_q1_callback)(const void*);
  uint64_t (*act_v_arh_q2_callback)(const void*);
  uint64_t (*act_v_arh_q3_callback)(const void*);
  uint64_t (*act_v_arh_q4_callback)(const void*);
  int16_t (*v_ar_aval_callback)(const void*);
  uint16_t (*v_ar_aval_sf_callback)(const void*);
  uint16_t (*w_aval_callback)(const void*);
  uint16_t (*w_aval_sf_callback)(const void*);
  uint32_t (*st_set_lim_msk_callback)(const void*);
  uint32_t (*st_act_ctl_callback)(const void*);
  const char *(*tm_src_callback)(const void*);
  uint32_t (*tms_callback)(const void*);
  uint16_t (*rt_st_callback)(const void*);
  uint16_t (*ris_callback)(const void*);
  uint16_t (*ris_sf_callback)(const void*);
} Model122CallbackAdapter;

typedef struct Model122StatefulAdapter {
  uint16_t pv_conn;
  uint16_t stor_conn;
  uint16_t ecp_conn;
  uint64_t act_wh;
  uint64_t act_v_ah;
  uint64_t act_v_arh_q1;
  uint64_t act_v_arh_q2;
  uint64_t act_v_arh_q3;
  uint64_t act_v_arh_q4;
  int16_t v_ar_aval;
  uint16_t v_ar_aval_sf;
  uint16_t w_aval;
  uint16_t w_aval_sf;
  uint32_t st_set_lim_msk;
  uint32_t st_act_ctl;
  char tm_src[8];
  uint32_t tms;
  uint16_t rt_st;
  uint16_t ris;
  uint16_t ris_sf;
} Model122StatefulAdapter;

typedef struct Model123CallbackAdapter {
  void *context;
  uint16_t (*conn_win_tms_callback)(const void*);
  void (*set_conn_win_tms_callback)(uint16_t, void*);
  uint16_t (*conn_rvrt_tms_callback)(const void*);
  void (*set_conn_rvrt_tms_callback)(uint16_t, void*);
  Conn (*conn_callback)(const void*);
  void (*set_conn_callback)(Conn, void*);
  uint16_t (*w_max_lim_pct_callback)(const void*);
  void (*set_w_max_lim_pct_callback)(uint16_t, void*);
  uint16_t (*w_max_lim_pct_win_tms_callback)(const void*);
  void (*set_w_max_lim_pct_win_tms_callback)(uint16_t, void*);
  uint16_t (*w_max_lim_pct_rvrt_tms_callback)(const void*);
  void (*set_w_max_lim_pct_rvrt_tms_callback)(uint16_t, void*);
  uint16_t (*w_max_lim_pct_rmp_tms_callback)(const void*);
  void (*set_w_max_lim_pct_rmp_tms_callback)(uint16_t, void*);
  WMaxLimEna (*w_max_lim_ena_callback)(const void*);
  void (*set_w_max_lim_ena_callback)(WMaxLimEna, void*);
  int16_t (*out_pf_set_callback)(const void*);
  void (*set_out_pf_set_callback)(int16_t, void*);
  uint16_t (*out_pf_set_win_tms_callback)(const void*);
  void (*set_out_pf_set_win_tms_callback)(uint16_t, void*);
  uint16_t (*out_pf_set_rvrt_tms_callback)(const void*);
  void (*set_out_pf_set_rvrt_tms_callback)(uint16_t, void*);
  uint16_t (*out_pf_set_rmp_tms_callback)(const void*);
  void (*set_out_pf_set_rmp_tms_callback)(uint16_t, void*);
  OutPfSetEna (*out_pf_set_ena_callback)(const void*);
  void (*set_out_pf_set_ena_callback)(OutPfSetEna, void*);
  int16_t (*v_ar_w_max_pct_callback)(const void*);
  void (*set_v_ar_w_max_pct_callback)(int16_t, void*);
  int16_t (*v_ar_max_pct_callback)(const void*);
  void (*set_v_ar_max_pct_callback)(int16_t, void*);
  int16_t (*v_ar_aval_pct_callback)(const void*);
  void (*set_v_ar_aval_pct_callback)(int16_t, void*);
  uint16_t (*v_ar_pct_win_tms_callback)(const void*);
  void (*set_v_ar_pct_win_tms_callback)(uint16_t, void*);
  uint16_t (*v_ar_pct_rvrt_tms_callback)(const void*);
  void (*set_v_ar_pct_rvrt_tms_callback)(uint16_t, void*);
  uint16_t (*v_ar_pct_rmp_tms_callback)(const void*);
  void (*set_v_ar_pct_rmp_tms_callback)(uint16_t, void*);
  VArPctMod (*v_ar_pct_mod_callback)(const void*);
  void (*set_v_ar_pct_mod_callback)(VArPctMod, void*);
  VArPctEna (*v_ar_pct_ena_callback)(const void*);
  void (*set_v_ar_pct_ena_callback)(VArPctEna, void*);
  uint16_t (*w_max_lim_pct_sf_callback)(const void*);
  uint16_t (*out_pf_set_sf_callback)(const void*);
  uint16_t (*v_ar_pct_sf_callback)(const void*);
} Model123CallbackAdapter;

typedef struct Model123StatefulAdapter {
  uint16_t conn_win_tms;
  uint16_t conn_rvrt_tms;
  Conn conn;
  uint16_t w_max_lim_pct;
  uint16_t w_max_lim_pct_win_tms;
  uint16_t w_max_lim_pct_rvrt_tms;
  uint16_t w_max_lim_pct_rmp_tms;
  WMaxLimEna w_max_lim_ena;
  int16_t out_pf_set;
  uint16_t out_pf_set_win_tms;
  uint16_t out_pf_set_rvrt_tms;
  uint16_t out_pf_set_rmp_tms;
  OutPfSetEna out_pf_set_ena;
  int16_t v_ar_w_max_pct;
  int16_t v_ar_max_pct;
  int16_t v_ar_aval_pct;
  uint16_t v_ar_pct_win_tms;
  uint16_t v_ar_pct_rvrt_tms;
  uint16_t v_ar_pct_rmp_tms;
  VArPctMod v_ar_pct_mod;
  VArPctEna v_ar_pct_ena;
  uint16_t w_max_lim_pct_sf;
  uint16_t out_pf_set_sf;
  uint16_t v_ar_pct_sf;
} Model123StatefulAdapter;

typedef struct Model124CallbackAdapter {
  void *context;
  uint16_t (*w_cha_max_callback)(const void*);
  void (*set_w_cha_max_callback)(uint16_t, void*);
  uint16_t (*w_cha_gra_callback)(const void*);
  void (*set_w_cha_gra_callback)(uint16_t, void*);
  uint16_t (*w_dis_cha_gra_callback)(const void*);
  void (*set_w_dis_cha_gra_callback)(uint16_t, void*);
  uint16_t (*stor_ctl_mod_callback)(const void*);
  void (*set_stor_ctl_mod_callback)(uint16_t, void*);
  uint16_t (*va_cha_max_callback)(const void*);
  void (*set_va_cha_max_callback)(uint16_t, void*);
  uint16_t (*min_rsv_pct_callback)(const void*);
  void (*set_min_rsv_pct_callback)(uint16_t, void*);
  uint16_t (*cha_state_callback)(const void*);
  uint16_t (*stor_aval_callback)(const void*);
  uint16_t (*in_bat_v_callback)(const void*);
  ChaSt (*cha_st_callback)(const void*);
  int16_t (*out_w_rte_callback)(const void*);
  void (*set_out_w_rte_callback)(int16_t, void*);
  int16_t (*in_w_rte_callback)(const void*);
  void (*set_in_w_rte_callback)(int16_t, void*);
  uint16_t (*in_out_w_rte_win_tms_callback)(const void*);
  void (*set_in_out_w_rte_win_tms_callback)(uint16_t, void*);
  uint16_t (*in_out_w_rte_rvrt_tms_callback)(const void*);
  void (*set_in_out_w_rte_rvrt_tms_callback)(uint16_t, void*);
  uint16_t (*in_out_w_rte_rmp_tms_callback)(const void*);
  void (*set_in_out_w_rte_rmp_tms_callback)(uint16_t, void*);
  ChaGriSet (*cha_gri_set_callback)(const void*);
  void (*set_cha_gri_set_callback)(ChaGriSet, void*);
  uint16_t (*w_cha_max_sf_callback)(const void*);
  uint16_t (*w_cha_dis_cha_gra_sf_callback)(const void*);
  uint16_t (*va_cha_max_sf_callback)(const void*);
  uint16_t (*min_rsv_pct_sf_callback)(const void*);
  uint16_t (*cha_state_sf_callback)(const void*);
  uint16_t (*stor_aval_sf_callback)(const void*);
  uint16_t (*in_bat_v_sf_callback)(const void*);
  uint16_t (*in_out_w_rte_sf_callback)(const void*);
} Model124CallbackAdapter;

typedef struct Model124StatefulAdapter {
  uint16_t w_cha_max;
  uint16_t w_cha_gra;
  uint16_t w_dis_cha_gra;
  uint16_t stor_ctl_mod;
  uint16_t va_cha_max;
  uint16_t min_rsv_pct;
  uint16_t cha_state;
  uint16_t stor_aval;
  uint16_t in_bat_v;
  ChaSt cha_st;
  int16_t out_w_rte;
  int16_t in_w_rte;
  uint16_t in_out_w_rte_win_tms;
  uint16_t in_out_w_rte_rvrt_tms;
  uint16_t in_out_w_rte_rmp_tms;
  ChaGriSet cha_gri_set;
  uint16_t w_cha_max_sf;
  uint16_t w_cha_dis_cha_gra_sf;
  uint16_t va_cha_max_sf;
  uint16_t min_rsv_pct_sf;
  uint16_t cha_state_sf;
  uint16_t stor_aval_sf;
  uint16_t in_bat_v_sf;
  uint16_t in_out_w_rte_sf;
} Model124StatefulAdapter;

typedef struct Model125CallbackAdapter {
  void *context;
  uint16_t (*mod_ena_callback)(const void*);
  void (*set_mod_ena_callback)(uint16_t, void*);
  SigType (*sig_type_callback)(const void*);
  void (*set_sig_type_callback)(SigType, void*);
  int16_t (*sig_callback)(const void*);
  void (*set_sig_callback)(int16_t, void*);
  uint16_t (*win_tms_callback)(const void*);
  void (*set_win_tms_callback)(uint16_t, void*);
  uint16_t (*rvt_tms_callback)(const void*);
  void (*set_rvt_tms_callback)(uint16_t, void*);
  uint16_t (*rmp_tms_callback)(const void*);
  void (*set_rmp_tms_callback)(uint16_t, void*);
  uint16_t (*sig_sf_callback)(const void*);
} Model125CallbackAdapter;

typedef struct Model125StatefulAdapter {
  uint16_t mod_ena;
  SigType sig_type;
  int16_t sig;
  uint16_t win_tms;
  uint16_t rvt_tms;
  uint16_t rmp_tms;
  uint16_t sig_sf;
} Model125StatefulAdapter;

typedef struct Model126CallbackAdapter {
  void *context;
  uint16_t (*act_crv_callback)(const void*);
  void (*set_act_crv_callback)(uint16_t, void*);
  uint16_t (*mod_ena_callback)(const void*);
  void (*set_mod_ena_callback)(uint16_t, void*);
  uint16_t (*win_tms_callback)(const void*);
  void (*set_win_tms_callback)(uint16_t, void*);
  uint16_t (*rvrt_tms_callback)(const void*);
  void (*set_rvrt_tms_callback)(uint16_t, void*);
  uint16_t (*rmp_tms_callback)(const void*);
  void (*set_rmp_tms_callback)(uint16_t, void*);
  uint16_t (*n_crv_callback)(const void*);
  uint16_t (*n_pt_callback)(const void*);
  uint16_t (*v_sf_callback)(const void*);
  uint16_t (*dept_ref_sf_callback)(const void*);
  uint16_t (*rmp_inc_dec_sf_callback)(const void*);
} Model126CallbackAdapter;

typedef struct Model126StatefulAdapter {
  uint16_t act_crv;
  uint16_t mod_ena;
  uint16_t win_tms;
  uint16_t rvrt_tms;
  uint16_t rmp_tms;
  uint16_t n_crv;
  uint16_t n_pt;
  uint16_t v_sf;
  uint16_t dept_ref_sf;
  uint16_t rmp_inc_dec_sf;
} Model126StatefulAdapter;

typedef struct Model127CallbackAdapter {
  void *context;
  uint16_t (*w_gra_callback)(const void*);
  void (*set_w_gra_callback)(uint16_t, void*);
  int16_t (*hz_str_callback)(const void*);
  void (*set_hz_str_callback)(int16_t, void*);
  int16_t (*hz_stop_callback)(const void*);
  void (*set_hz_stop_callback)(int16_t, void*);
  uint16_t (*hys_ena_callback)(const void*);
  void (*set_hys_ena_callback)(uint16_t, void*);
  uint16_t (*mod_ena_callback)(const void*);
  void (*set_mod_ena_callback)(uint16_t, void*);
  uint16_t (*hz_stop_w_gra_callback)(const void*);
  void (*set_hz_stop_w_gra_callback)(uint16_t, void*);
  uint16_t (*w_gra_sf_callback)(const void*);
  uint16_t (*hz_str_stop_sf_callback)(const void*);
  uint16_t (*rmp_inc_dec_sf_callback)(const void*);
} Model127CallbackAdapter;

typedef struct Model127StatefulAdapter {
  uint16_t w_gra;
  int16_t hz_str;
  int16_t hz_stop;
  uint16_t hys_ena;
  uint16_t mod_ena;
  uint16_t hz_stop_w_gra;
  uint16_t w_gra_sf;
  uint16_t hz_str_stop_sf;
  uint16_t rmp_inc_dec_sf;
} Model127StatefulAdapter;

typedef struct Model128CallbackAdapter {
  void *context;
  ArGraMod (*ar_gra_mod_callback)(const void*);
  void (*set_ar_gra_mod_callback)(ArGraMod, void*);
  uint16_t (*ar_gra_sag_callback)(const void*);
  void (*set_ar_gra_sag_callback)(uint16_t, void*);
  uint16_t (*ar_gra_swell_callback)(const void*);
  void (*set_ar_gra_swell_callback)(uint16_t, void*);
  uint16_t (*mod_ena_callback)(const void*);
  void (*set_mod_ena_callback)(uint16_t, void*);
  uint16_t (*fil_tms_callback)(const void*);
  void (*set_fil_tms_callback)(uint16_t, void*);
  uint16_t (*db_v_min_callback)(const void*);
  void (*set_db_v_min_callback)(uint16_t, void*);
  uint16_t (*db_v_max_callback)(const void*);
  void (*set_db_v_max_callback)(uint16_t, void*);
  uint16_t (*blk_zn_v_callback)(const void*);
  void (*set_blk_zn_v_callback)(uint16_t, void*);
  uint16_t (*hys_blk_zn_v_callback)(const void*);
  void (*set_hys_blk_zn_v_callback)(uint16_t, void*);
  uint16_t (*blk_zn_tmms_callback)(const void*);
  void (*set_blk_zn_tmms_callback)(uint16_t, void*);
  uint16_t (*hold_tmms_callback)(const void*);
  void (*set_hold_tmms_callback)(uint16_t, void*);
  uint16_t (*ar_gra_sf_callback)(const void*);
  uint16_t (*v_ref_pct_sf_callback)(const void*);
} Model128CallbackAdapter;

typedef struct Model128StatefulAdapter {
  ArGraMod ar_gra_mod;
  uint16_t ar_gra_sag;
  uint16_t ar_gra_swell;
  uint16_t mod_ena;
  uint16_t fil_tms;
  uint16_t db_v_min;
  uint16_t db_v_max;
  uint16_t blk_zn_v;
  uint16_t hys_blk_zn_v;
  uint16_t blk_zn_tmms;
  uint16_t hold_tmms;
  uint16_t ar_gra_sf;
  uint16_t v_ref_pct_sf;
} Model128StatefulAdapter;

typedef struct Model129CallbackAdapter {
  void *context;
  uint16_t (*act_crv_callback)(const void*);
  void (*set_act_crv_callback)(uint16_t, void*);
  uint16_t (*mod_ena_callback)(const void*);
  void (*set_mod_ena_callback)(uint16_t, void*);
  uint16_t (*win_tms_callback)(const void*);
  void (*set_win_tms_callback)(uint16_t, void*);
  uint16_t (*rvrt_tms_callback)(const void*);
  void (*set_rvrt_tms_callback)(uint16_t, void*);
  uint16_t (*rmp_tms_callback)(const void*);
  void (*set_rmp_tms_callback)(uint16_t, void*);
  uint16_t (*n_crv_callback)(const void*);
  uint16_t (*n_pt_callback)(const void*);
  uint16_t (*tms_sf_callback)(const void*);
  uint16_t (*v_sf_callback)(const void*);
} Model129CallbackAdapter;

typedef struct Model129StatefulAdapter {
  uint16_t act_crv;
  uint16_t mod_ena;
  uint16_t win_tms;
  uint16_t rvrt_tms;
  uint16_t rmp_tms;
  uint16_t n_crv;
  uint16_t n_pt;
  uint16_t tms_sf;
  uint16_t v_sf;
} Model129StatefulAdapter;

typedef struct Model130CallbackAdapter {
  void *context;
  uint16_t (*act_crv_callback)(const void*);
  void (*set_act_crv_callback)(uint16_t, void*);
  uint16_t (*mod_ena_callback)(const void*);
  void (*set_mod_ena_callback)(uint16_t, void*);
  uint16_t (*win_tms_callback)(const void*);
  void (*set_win_tms_callback)(uint16_t, void*);
  uint16_t (*rvrt_tms_callback)(const void*);
  void (*set_rvrt_tms_callback)(uint16_t, void*);
  uint16_t (*rmp_tms_callback)(const void*);
  void (*set_rmp_tms_callback)(uint16_t, void*);
  uint16_t (*n_crv_callback)(const void*);
  uint16_t (*n_pt_callback)(const void*);
  uint16_t (*tms_sf_callback)(const void*);
  uint16_t (*v_sf_callback)(const void*);
} Model130CallbackAdapter;

typedef struct Model130StatefulAdapter {
  uint16_t act_crv;
  uint16_t mod_ena;
  uint16_t win_tms;
  uint16_t rvrt_tms;
  uint16_t rmp_tms;
  uint16_t n_crv;
  uint16_t n_pt;
  uint16_t tms_sf;
  uint16_t v_sf;
} Model130StatefulAdapter;

typedef struct Model131CallbackAdapter {
  void *context;
  uint16_t (*act_crv_callback)(const void*);
  void (*set_act_crv_callback)(uint16_t, void*);
  uint16_t (*mod_ena_callback)(const void*);
  void (*set_mod_ena_callback)(uint16_t, void*);
  uint16_t (*win_tms_callback)(const void*);
  void (*set_win_tms_callback)(uint16_t, void*);
  uint16_t (*rvrt_tms_callback)(const void*);
  void (*set_rvrt_tms_callback)(uint16_t, void*);
  uint16_t (*rmp_tms_callback)(const void*);
  void (*set_rmp_tms_callback)(uint16_t, void*);
  uint16_t (*n_crv_callback)(const void*);
  uint16_t (*n_pt_callback)(const void*);
  uint16_t (*w_sf_callback)(const void*);
  uint16_t (*pf_sf_callback)(const void*);
  uint16_t (*rmp_inc_dec_sf_callback)(const void*);
} Model131CallbackAdapter;

typedef struct Model131StatefulAdapter {
  uint16_t act_crv;
  uint16_t mod_ena;
  uint16_t win_tms;
  uint16_t rvrt_tms;
  uint16_t rmp_tms;
  uint16_t n_crv;
  uint16_t n_pt;
  uint16_t w_sf;
  uint16_t pf_sf;
  uint16_t rmp_inc_dec_sf;
} Model131StatefulAdapter;

typedef struct Model132CallbackAdapter {
  void *context;
  uint16_t (*act_crv_callback)(const void*);
  void (*set_act_crv_callback)(uint16_t, void*);
  uint16_t (*mod_ena_callback)(const void*);
  void (*set_mod_ena_callback)(uint16_t, void*);
  uint16_t (*win_tms_callback)(const void*);
  void (*set_win_tms_callback)(uint16_t, void*);
  uint16_t (*rvrt_tms_callback)(const void*);
  void (*set_rvrt_tms_callback)(uint16_t, void*);
  uint16_t (*rmp_tms_callback)(const void*);
  void (*set_rmp_tms_callback)(uint16_t, void*);
  uint16_t (*n_crv_callback)(const void*);
  uint16_t (*n_pt_callback)(const void*);
  uint16_t (*v_sf_callback)(const void*);
  uint16_t (*dept_ref_sf_callback)(const void*);
  uint16_t (*rmp_inc_dec_sf_callback)(const void*);
} Model132CallbackAdapter;

typedef struct Model132StatefulAdapter {
  uint16_t act_crv;
  uint16_t mod_ena;
  uint16_t win_tms;
  uint16_t rvrt_tms;
  uint16_t rmp_tms;
  uint16_t n_crv;
  uint16_t n_pt;
  uint16_t v_sf;
  uint16_t dept_ref_sf;
  uint16_t rmp_inc_dec_sf;
} Model132StatefulAdapter;

typedef struct Model133CallbackAdapter {
  void *context;
  uint32_t (*act_schd_callback)(const void*);
  void (*set_act_schd_callback)(uint32_t, void*);
  uint16_t (*mod_ena_callback)(const void*);
  void (*set_mod_ena_callback)(uint16_t, void*);
  uint16_t (*n_schd_callback)(const void*);
  uint16_t (*n_pts_callback)(const void*);
} Model133CallbackAdapter;

typedef struct Model133StatefulAdapter {
  uint32_t act_schd;
  uint16_t mod_ena;
  uint16_t n_schd;
  uint16_t n_pts;
} Model133StatefulAdapter;

typedef struct Model134CallbackAdapter {
  void *context;
  uint16_t (*act_crv_callback)(const void*);
  void (*set_act_crv_callback)(uint16_t, void*);
  uint16_t (*mod_ena_callback)(const void*);
  void (*set_mod_ena_callback)(uint16_t, void*);
  uint16_t (*win_tms_callback)(const void*);
  void (*set_win_tms_callback)(uint16_t, void*);
  uint16_t (*rvrt_tms_callback)(const void*);
  void (*set_rvrt_tms_callback)(uint16_t, void*);
  uint16_t (*rmp_tms_callback)(const void*);
  void (*set_rmp_tms_callback)(uint16_t, void*);
  uint16_t (*n_crv_callback)(const void*);
  uint16_t (*n_pt_callback)(const void*);
  uint16_t (*hz_sf_callback)(const void*);
  uint16_t (*w_sf_callback)(const void*);
  uint16_t (*rmp_inc_dec_sf_callback)(const void*);
} Model134CallbackAdapter;

typedef struct Model134StatefulAdapter {
  uint16_t act_crv;
  uint16_t mod_ena;
  uint16_t win_tms;
  uint16_t rvrt_tms;
  uint16_t rmp_tms;
  uint16_t n_crv;
  uint16_t n_pt;
  uint16_t hz_sf;
  uint16_t w_sf;
  uint16_t rmp_inc_dec_sf;
} Model134StatefulAdapter;

typedef struct Model135CallbackAdapter {
  void *context;
  uint16_t (*act_crv_callback)(const void*);
  void (*set_act_crv_callback)(uint16_t, void*);
  uint16_t (*mod_ena_callback)(const void*);
  void (*set_mod_ena_callback)(uint16_t, void*);
  uint16_t (*win_tms_callback)(const void*);
  void (*set_win_tms_callback)(uint16_t, void*);
  uint16_t (*rvrt_tms_callback)(const void*);
  void (*set_rvrt_tms_callback)(uint16_t, void*);
  uint16_t (*rmp_tms_callback)(const void*);
  void (*set_rmp_tms_callback)(uint16_t, void*);
  uint16_t (*n_crv_callback)(const void*);
  uint16_t (*n_pt_callback)(const void*);
  uint16_t (*tms_sf_callback)(const void*);
  uint16_t (*hz_sf_callback)(const void*);
} Model135CallbackAdapter;

typedef struct Model135StatefulAdapter {
  uint16_t act_crv;
  uint16_t mod_ena;
  uint16_t win_tms;
  uint16_t rvrt_tms;
  uint16_t rmp_tms;
  uint16_t n_crv;
  uint16_t n_pt;
  uint16_t tms_sf;
  uint16_t hz_sf;
} Model135StatefulAdapter;

typedef struct Model136CallbackAdapter {
  void *context;
  uint16_t (*act_crv_callback)(const void*);
  void (*set_act_crv_callback)(uint16_t, void*);
  uint16_t (*mod_ena_callback)(const void*);
  void (*set_mod_ena_callback)(uint16_t, void*);
  uint16_t (*win_tms_callback)(const void*);
  void (*set_win_tms_callback)(uint16_t, void*);
  uint16_t (*rvrt_tms_callback)(const void*);
  void (*set_rvrt_tms_callback)(uint16_t, void*);
  uint16_t (*rmp_tms_callback)(const void*);
  void (*set_rmp_tms_callback)(uint16_t, void*);
  uint16_t (*n_crv_callback)(const void*);
  uint16_t (*n_pt_callback)(const void*);
  uint16_t (*tms_sf_callback)(const void*);
  uint16_t (*hz_sf_callback)(const void*);
} Model136CallbackAdapter;

typedef struct Model136StatefulAdapter {
  uint16_t act_crv;
  uint16_t mod_ena;
  uint16_t win_tms;
  uint16_t rvrt_tms;
  uint16_t rmp_tms;
  uint16_t n_crv;
  uint16_t n_pt;
  uint16_t tms_sf;
  uint16_t hz_sf;
} Model136StatefulAdapter;

typedef struct Model137CallbackAdapter {
  void *context;
  uint16_t (*act_crv_callback)(const void*);
  void (*set_act_crv_callback)(uint16_t, void*);
  uint16_t (*mod_ena_callback)(const void*);
  void (*set_mod_ena_callback)(uint16_t, void*);
  uint16_t (*win_tms_callback)(const void*);
  void (*set_win_tms_callback)(uint16_t, void*);
  uint16_t (*rvrt_tms_callback)(const void*);
  void (*set_rvrt_tms_callback)(uint16_t, void*);
  uint16_t (*rmp_tms_callback)(const void*);
  void (*set_rmp_tms_callback)(uint16_t, void*);
  uint16_t (*n_crv_callback)(const void*);
  uint16_t (*n_pt_callback)(const void*);
  uint16_t (*tms_sf_callback)(const void*);
  uint16_t (*v_sf_callback)(const void*);
} Model137CallbackAdapter;

typedef struct Model137StatefulAdapter {
  uint16_t act_crv;
  uint16_t mod_ena;
  uint16_t win_tms;
  uint16_t rvrt_tms;
  uint16_t rmp_tms;
  uint16_t n_crv;
  uint16_t n_pt;
  uint16_t tms_sf;
  uint16_t v_sf;
} Model137StatefulAdapter;

typedef struct Model138CallbackAdapter {
  void *context;
  uint16_t (*act_crv_callback)(const void*);
  void (*set_act_crv_callback)(uint16_t, void*);
  uint16_t (*mod_ena_callback)(const void*);
  void (*set_mod_ena_callback)(uint16_t, void*);
  uint16_t (*win_tms_callback)(const void*);
  void (*set_win_tms_callback)(uint16_t, void*);
  uint16_t (*rvrt_tms_callback)(const void*);
  void (*set_rvrt_tms_callback)(uint16_t, void*);
  uint16_t (*rmp_tms_callback)(const void*);
  void (*set_rmp_tms_callback)(uint16_t, void*);
  uint16_t (*n_crv_callback)(const void*);
  uint16_t (*n_pt_callback)(const void*);
  uint16_t (*tms_sf_callback)(const void*);
  uint16_t (*v_sf_callback)(const void*);
} Model138CallbackAdapter;

typedef struct Model138StatefulAdapter {
  uint16_t act_crv;
  uint16_t mod_ena;
  uint16_t win_tms;
  uint16_t rvrt_tms;
  uint16_t rmp_tms;
  uint16_t n_crv;
  uint16_t n_pt;
  uint16_t tms_sf;
  uint16_t v_sf;
} Model138StatefulAdapter;

typedef struct Model139CallbackAdapter {
  void *context;
  uint16_t (*act_crv_callback)(const void*);
  void (*set_act_crv_callback)(uint16_t, void*);
  uint16_t (*mod_ena_callback)(const void*);
  void (*set_mod_ena_callback)(uint16_t, void*);
  uint16_t (*win_tms_callback)(const void*);
  void (*set_win_tms_callback)(uint16_t, void*);
  uint16_t (*rvrt_tms_callback)(const void*);
  void (*set_rvrt_tms_callback)(uint16_t, void*);
  uint16_t (*rmp_tms_callback)(const void*);
  void (*set_rmp_tms_callback)(uint16_t, void*);
  uint16_t (*n_crv_callback)(const void*);
  uint16_t (*n_pt_callback)(const void*);
  uint16_t (*tms_sf_callback)(const void*);
  uint16_t (*v_sf_callback)(const void*);
  CrvType (*crv_type_callback)(const void*);
} Model139CallbackAdapter;

typedef struct Model139StatefulAdapter {
  uint16_t act_crv;
  uint16_t mod_ena;
  uint16_t win_tms;
  uint16_t rvrt_tms;
  uint16_t rmp_tms;
  uint16_t n_crv;
  uint16_t n_pt;
  uint16_t tms_sf;
  uint16_t v_sf;
  CrvType crv_type;
} Model139StatefulAdapter;

typedef struct Model140CallbackAdapter {
  void *context;
  uint16_t (*act_crv_callback)(const void*);
  void (*set_act_crv_callback)(uint16_t, void*);
  uint16_t (*mod_ena_callback)(const void*);
  void (*set_mod_ena_callback)(uint16_t, void*);
  uint16_t (*win_tms_callback)(const void*);
  void (*set_win_tms_callback)(uint16_t, void*);
  uint16_t (*rvrt_tms_callback)(const void*);
  void (*set_rvrt_tms_callback)(uint16_t, void*);
  uint16_t (*rmp_tms_callback)(const void*);
  void (*set_rmp_tms_callback)(uint16_t, void*);
  uint16_t (*n_crv_callback)(const void*);
  uint16_t (*n_pt_callback)(const void*);
  uint16_t (*tms_sf_callback)(const void*);
  uint16_t (*v_sf_callback)(const void*);
  CrvType (*crv_type_callback)(const void*);
} Model140CallbackAdapter;

typedef struct Model140StatefulAdapter {
  uint16_t act_crv;
  uint16_t mod_ena;
  uint16_t win_tms;
  uint16_t rvrt_tms;
  uint16_t rmp_tms;
  uint16_t n_crv;
  uint16_t n_pt;
  uint16_t tms_sf;
  uint16_t v_sf;
  CrvType crv_type;
} Model140StatefulAdapter;

typedef struct Model141CallbackAdapter {
  void *context;
  uint16_t (*act_crv_callback)(const void*);
  void (*set_act_crv_callback)(uint16_t, void*);
  uint16_t (*mod_ena_callback)(const void*);
  void (*set_mod_ena_callback)(uint16_t, void*);
  uint16_t (*win_tms_callback)(const void*);
  void (*set_win_tms_callback)(uint16_t, void*);
  uint16_t (*rvrt_tms_callback)(const void*);
  void (*set_rvrt_tms_callback)(uint16_t, void*);
  uint16_t (*rmp_tms_callback)(const void*);
  void (*set_rmp_tms_callback)(uint16_t, void*);
  uint16_t (*n_crv_callback)(const void*);
  uint16_t (*n_pt_callback)(const void*);
  uint16_t (*tms_sf_callback)(const void*);
  uint16_t (*hz_sf_callback)(const void*);
} Model141CallbackAdapter;

typedef struct Model141StatefulAdapter {
  uint16_t act_crv;
  uint16_t mod_ena;
  uint16_t win_tms;
  uint16_t rvrt_tms;
  uint16_t rmp_tms;
  uint16_t n_crv;
  uint16_t n_pt;
  uint16_t tms_sf;
  uint16_t hz_sf;
} Model141StatefulAdapter;

typedef struct Model142CallbackAdapter {
  void *context;
  uint16_t (*act_crv_callback)(const void*);
  void (*set_act_crv_callback)(uint16_t, void*);
  uint16_t (*mod_ena_callback)(const void*);
  void (*set_mod_ena_callback)(uint16_t, void*);
  uint16_t (*win_tms_callback)(const void*);
  void (*set_win_tms_callback)(uint16_t, void*);
  uint16_t (*rvrt_tms_callback)(const void*);
  void (*set_rvrt_tms_callback)(uint16_t, void*);
  uint16_t (*rmp_tms_callback)(const void*);
  void (*set_rmp_tms_callback)(uint16_t, void*);
  uint16_t (*n_crv_callback)(const void*);
  uint16_t (*n_pt_callback)(const void*);
  uint16_t (*tms_sf_callback)(const void*);
  uint16_t (*hz_sf_callback)(const void*);
} Model142CallbackAdapter;

typedef struct Model142StatefulAdapter {
  uint16_t act_crv;
  uint16_t mod_ena;
  uint16_t win_tms;
  uint16_t rvrt_tms;
  uint16_t rmp_tms;
  uint16_t n_crv;
  uint16_t n_pt;
  uint16_t tms_sf;
  uint16_t hz_sf;
} Model142StatefulAdapter;

typedef struct Model143CallbackAdapter {
  void *context;
  uint16_t (*act_crv_callback)(const void*);
  void (*set_act_crv_callback)(uint16_t, void*);
  uint16_t (*mod_ena_callback)(const void*);
  void (*set_mod_ena_callback)(uint16_t, void*);
  uint16_t (*win_tms_callback)(const void*);
  void (*set_win_tms_callback)(uint16_t, void*);
  uint16_t (*rvrt_tms_callback)(const void*);
  void (*set_rvrt_tms_callback)(uint16_t, void*);
  uint16_t (*rmp_tms_callback)(const void*);
  void (*set_rmp_tms_callback)(uint16_t, void*);
  uint16_t (*n_crv_callback)(const void*);
  uint16_t (*n_pt_callback)(const void*);
  uint16_t (*tms_sf_callback)(const void*);
  uint16_t (*hz_sf_callback)(const void*);
  CrvType (*crv_type_callback)(const void*);
} Model143CallbackAdapter;

typedef struct Model143StatefulAdapter {
  uint16_t act_crv;
  uint16_t mod_ena;
  uint16_t win_tms;
  uint16_t rvrt_tms;
  uint16_t rmp_tms;
  uint16_t n_crv;
  uint16_t n_pt;
  uint16_t tms_sf;
  uint16_t hz_sf;
  CrvType crv_type;
} Model143StatefulAdapter;

typedef struct Model144CallbackAdapter {
  void *context;
  uint16_t (*act_crv_callback)(const void*);
  void (*set_act_crv_callback)(uint16_t, void*);
  uint16_t (*mod_ena_callback)(const void*);
  void (*set_mod_ena_callback)(uint16_t, void*);
  uint16_t (*win_tms_callback)(const void*);
  void (*set_win_tms_callback)(uint16_t, void*);
  uint16_t (*rvrt_tms_callback)(const void*);
  void (*set_rvrt_tms_callback)(uint16_t, void*);
  uint16_t (*rmp_tms_callback)(const void*);
  void (*set_rmp_tms_callback)(uint16_t, void*);
  uint16_t (*n_crv_callback)(const void*);
  uint16_t (*n_pt_callback)(const void*);
  uint16_t (*tms_sf_callback)(const void*);
  uint16_t (*hz_sf_callback)(const void*);
  CrvType (*crv_type_callback)(const void*);
} Model144CallbackAdapter;

typedef struct Model144StatefulAdapter {
  uint16_t act_crv;
  uint16_t mod_ena;
  uint16_t win_tms;
  uint16_t rvrt_tms;
  uint16_t rmp_tms;
  uint16_t n_crv;
  uint16_t n_pt;
  uint16_t tms_sf;
  uint16_t hz_sf;
  CrvType crv_type;
} Model144StatefulAdapter;

typedef struct Model145CallbackAdapter {
  void *context;
  uint16_t (*ramp_up_rate_callback)(const void*);
  void (*set_ramp_up_rate_callback)(uint16_t, void*);
  uint16_t (*nom_rmp_dn_rte_callback)(const void*);
  void (*set_nom_rmp_dn_rte_callback)(uint16_t, void*);
  uint16_t (*emergency_ramp_up_rate_callback)(const void*);
  void (*set_emergency_ramp_up_rate_callback)(uint16_t, void*);
  uint16_t (*emergency_ramp_down_rate_callback)(const void*);
  void (*set_emergency_ramp_down_rate_callback)(uint16_t, void*);
  uint16_t (*connect_ramp_up_rate_callback)(const void*);
  void (*set_connect_ramp_up_rate_callback)(uint16_t, void*);
  uint16_t (*connect_ramp_down_rate_callback)(const void*);
  void (*set_connect_ramp_down_rate_callback)(uint16_t, void*);
  uint16_t (*default_ramp_rate_callback)(const void*);
  void (*set_default_ramp_rate_callback)(uint16_t, void*);
  uint16_t (*ramp_rate_scale_factor_callback)(const void*);
} Model145CallbackAdapter;

typedef struct Model145StatefulAdapter {
  uint16_t ramp_up_rate;
  uint16_t nom_rmp_dn_rte;
  uint16_t emergency_ramp_up_rate;
  uint16_t emergency_ramp_down_rate;
  uint16_t connect_ramp_up_rate;
  uint16_t connect_ramp_down_rate;
  uint16_t default_ramp_rate;
  uint16_t ramp_rate_scale_factor;
} Model145StatefulAdapter;

typedef struct Model160CallbackAdapter {
  void *context;
  uint16_t (*current_scale_factor_callback)(const void*);
  uint16_t (*voltage_scale_factor_callback)(const void*);
  uint16_t (*power_scale_factor_callback)(const void*);
  uint16_t (*energy_scale_factor_callback)(const void*);
  uint32_t (*global_events_callback)(const void*);
  uint16_t (*number_of_modules_callback)(const void*);
  uint16_t (*timestamp_period_callback)(const void*);
} Model160CallbackAdapter;

typedef struct Model160StatefulAdapter {
  uint16_t current_scale_factor;
  uint16_t voltage_scale_factor;
  uint16_t power_scale_factor;
  uint16_t energy_scale_factor;
  uint32_t global_events;
  uint16_t number_of_modules;
  uint16_t timestamp_period;
} Model160StatefulAdapter;

typedef struct Model201CallbackAdapter {
  void *context;
  int16_t (*amps_callback)(const void*);
  int16_t (*amps_phase_a_callback)(const void*);
  int16_t (*amps_phase_b_callback)(const void*);
  int16_t (*amps_phase_c_callback)(const void*);
  uint16_t (*a_sf_callback)(const void*);
  int16_t (*voltage_ln_callback)(const void*);
  int16_t (*phase_voltage_an_callback)(const void*);
  int16_t (*phase_voltage_bn_callback)(const void*);
  int16_t (*phase_voltage_cn_callback)(const void*);
  int16_t (*voltage_ll_callback)(const void*);
  int16_t (*phase_voltage_ab_callback)(const void*);
  int16_t (*phase_voltage_bc_callback)(const void*);
  int16_t (*phase_voltage_ca_callback)(const void*);
  uint16_t (*v_sf_callback)(const void*);
  int16_t (*hz_callback)(const void*);
  uint16_t (*hz_sf_callback)(const void*);
  int16_t (*watts_callback)(const void*);
  int16_t (*watts_phase_a_callback)(const void*);
  int16_t (*watts_phase_b_callback)(const void*);
  int16_t (*watts_phase_c_callback)(const void*);
  uint16_t (*w_sf_callback)(const void*);
  int16_t (*va_callback)(const void*);
  int16_t (*va_phase_a_callback)(const void*);
  int16_t (*va_phase_b_callback)(const void*);
  int16_t (*va_phase_c_callback)(const void*);
  uint16_t (*va_sf_callback)(const void*);
  int16_t (*var_callback)(const void*);
  int16_t (*var_phase_a_callback)(const void*);
  int16_t (*var_phase_b_callback)(const void*);
  int16_t (*var_phase_c_callback)(const void*);
  uint16_t (*var_sf_callback)(const void*);
  int16_t (*pf_callback)(const void*);
  int16_t (*pf_phase_a_callback)(const void*);
  int16_t (*pf_phase_b_callback)(const void*);
  int16_t (*pf_phase_c_callback)(const void*);
  uint16_t (*pf_sf_callback)(const void*);
  uint32_t (*total_watt_hours_exported_callback)(const void*);
  uint32_t (*total_watt_hours_exported_phase_a_callback)(const void*);
  uint32_t (*total_watt_hours_exported_phase_b_callback)(const void*);
  uint32_t (*total_watt_hours_exported_phase_c_callback)(const void*);
  uint32_t (*total_watt_hours_imported_callback)(const void*);
  uint32_t (*total_watt_hours_imported_phase_a_callback)(const void*);
  uint32_t (*total_watt_hours_imported_phase_b_callback)(const void*);
  uint32_t (*total_watt_hours_imported_phase_c_callback)(const void*);
  uint16_t (*tot_wh_sf_callback)(const void*);
  uint32_t (*total_va_hours_exported_callback)(const void*);
  uint32_t (*total_va_hours_exported_phase_a_callback)(const void*);
  uint32_t (*total_va_hours_exported_phase_b_callback)(const void*);
  uint32_t (*total_va_hours_exported_phase_c_callback)(const void*);
  uint32_t (*total_va_hours_imported_callback)(const void*);
  uint32_t (*total_va_hours_imported_phase_a_callback)(const void*);
  uint32_t (*total_va_hours_imported_phase_b_callback)(const void*);
  uint32_t (*total_va_hours_imported_phase_c_callback)(const void*);
  uint16_t (*tot_v_ah_sf_callback)(const void*);
  uint32_t (*total_var_hours_imported_q1_callback)(const void*);
  uint32_t (*total_v_ar_hours_imported_q1_phase_a_callback)(const void*);
  uint32_t (*total_v_ar_hours_imported_q1_phase_b_callback)(const void*);
  uint32_t (*total_v_ar_hours_imported_q1_phase_c_callback)(const void*);
  uint32_t (*total_v_ar_hours_imported_q2_callback)(const void*);
  uint32_t (*total_v_ar_hours_imported_q2_phase_a_callback)(const void*);
  uint32_t (*total_v_ar_hours_imported_q2_phase_b_callback)(const void*);
  uint32_t (*total_v_ar_hours_imported_q2_phase_c_callback)(const void*);
  uint32_t (*total_v_ar_hours_exported_q3_callback)(const void*);
  uint32_t (*total_v_ar_hours_exported_q3_phase_a_callback)(const void*);
  uint32_t (*total_v_ar_hours_exported_q3_phase_b_callback)(const void*);
  uint32_t (*total_v_ar_hours_exported_q3_phase_c_callback)(const void*);
  uint32_t (*total_v_ar_hours_exported_q4_callback)(const void*);
  uint32_t (*total_v_ar_hours_exported_q4_imported_phase_a_callback)(const void*);
  uint32_t (*total_v_ar_hours_exported_q4_imported_phase_b_callback)(const void*);
  uint32_t (*total_v_ar_hours_exported_q4_imported_phase_c_callback)(const void*);
  uint16_t (*tot_v_arh_sf_callback)(const void*);
  uint32_t (*events_callback)(const void*);
} Model201CallbackAdapter;

typedef struct Model201StatefulAdapter {
  int16_t amps;
  int16_t amps_phase_a;
  int16_t amps_phase_b;
  int16_t amps_phase_c;
  uint16_t a_sf;
  int16_t voltage_ln;
  int16_t phase_voltage_an;
  int16_t phase_voltage_bn;
  int16_t phase_voltage_cn;
  int16_t voltage_ll;
  int16_t phase_voltage_ab;
  int16_t phase_voltage_bc;
  int16_t phase_voltage_ca;
  uint16_t v_sf;
  int16_t hz;
  uint16_t hz_sf;
  int16_t watts;
  int16_t watts_phase_a;
  int16_t watts_phase_b;
  int16_t watts_phase_c;
  uint16_t w_sf;
  int16_t va;
  int16_t va_phase_a;
  int16_t va_phase_b;
  int16_t va_phase_c;
  uint16_t va_sf;
  int16_t var;
  int16_t var_phase_a;
  int16_t var_phase_b;
  int16_t var_phase_c;
  uint16_t var_sf;
  int16_t pf;
  int16_t pf_phase_a;
  int16_t pf_phase_b;
  int16_t pf_phase_c;
  uint16_t pf_sf;
  uint32_t total_watt_hours_exported;
  uint32_t total_watt_hours_exported_phase_a;
  uint32_t total_watt_hours_exported_phase_b;
  uint32_t total_watt_hours_exported_phase_c;
  uint32_t total_watt_hours_imported;
  uint32_t total_watt_hours_imported_phase_a;
  uint32_t total_watt_hours_imported_phase_b;
  uint32_t total_watt_hours_imported_phase_c;
  uint16_t tot_wh_sf;
  uint32_t total_va_hours_exported;
  uint32_t total_va_hours_exported_phase_a;
  uint32_t total_va_hours_exported_phase_b;
  uint32_t total_va_hours_exported_phase_c;
  uint32_t total_va_hours_imported;
  uint32_t total_va_hours_imported_phase_a;
  uint32_t total_va_hours_imported_phase_b;
  uint32_t total_va_hours_imported_phase_c;
  uint16_t tot_v_ah_sf;
  uint32_t total_var_hours_imported_q1;
  uint32_t total_v_ar_hours_imported_q1_phase_a;
  uint32_t total_v_ar_hours_imported_q1_phase_b;
  uint32_t total_v_ar_hours_imported_q1_phase_c;
  uint32_t total_v_ar_hours_imported_q2;
  uint32_t total_v_ar_hours_imported_q2_phase_a;
  uint32_t total_v_ar_hours_imported_q2_phase_b;
  uint32_t total_v_ar_hours_imported_q2_phase_c;
  uint32_t total_v_ar_hours_exported_q3;
  uint32_t total_v_ar_hours_exported_q3_phase_a;
  uint32_t total_v_ar_hours_exported_q3_phase_b;
  uint32_t total_v_ar_hours_exported_q3_phase_c;
  uint32_t total_v_ar_hours_exported_q4;
  uint32_t total_v_ar_hours_exported_q4_imported_phase_a;
  uint32_t total_v_ar_hours_exported_q4_imported_phase_b;
  uint32_t total_v_ar_hours_exported_q4_imported_phase_c;
  uint16_t tot_v_arh_sf;
  uint32_t events;
} Model201StatefulAdapter;

typedef struct Model202CallbackAdapter {
  void *context;
  int16_t (*amps_callback)(const void*);
  int16_t (*amps_phase_a_callback)(const void*);
  int16_t (*amps_phase_b_callback)(const void*);
  int16_t (*amps_phase_c_callback)(const void*);
  uint16_t (*a_sf_callback)(const void*);
  int16_t (*voltage_ln_callback)(const void*);
  int16_t (*phase_voltage_an_callback)(const void*);
  int16_t (*phase_voltage_bn_callback)(const void*);
  int16_t (*phase_voltage_cn_callback)(const void*);
  int16_t (*voltage_ll_callback)(const void*);
  int16_t (*phase_voltage_ab_callback)(const void*);
  int16_t (*phase_voltage_bc_callback)(const void*);
  int16_t (*phase_voltage_ca_callback)(const void*);
  uint16_t (*v_sf_callback)(const void*);
  int16_t (*hz_callback)(const void*);
  uint16_t (*hz_sf_callback)(const void*);
  int16_t (*watts_callback)(const void*);
  int16_t (*watts_phase_a_callback)(const void*);
  int16_t (*watts_phase_b_callback)(const void*);
  int16_t (*watts_phase_c_callback)(const void*);
  uint16_t (*w_sf_callback)(const void*);
  int16_t (*va_callback)(const void*);
  int16_t (*va_phase_a_callback)(const void*);
  int16_t (*va_phase_b_callback)(const void*);
  int16_t (*va_phase_c_callback)(const void*);
  uint16_t (*va_sf_callback)(const void*);
  int16_t (*var_callback)(const void*);
  int16_t (*var_phase_a_callback)(const void*);
  int16_t (*var_phase_b_callback)(const void*);
  int16_t (*var_phase_c_callback)(const void*);
  uint16_t (*var_sf_callback)(const void*);
  int16_t (*pf_callback)(const void*);
  int16_t (*pf_phase_a_callback)(const void*);
  int16_t (*pf_phase_b_callback)(const void*);
  int16_t (*pf_phase_c_callback)(const void*);
  uint16_t (*pf_sf_callback)(const void*);
  uint32_t (*total_watt_hours_exported_callback)(const void*);
  uint32_t (*total_watt_hours_exported_phase_a_callback)(const void*);
  uint32_t (*total_watt_hours_exported_phase_b_callback)(const void*);
  uint32_t (*total_watt_hours_exported_phase_c_callback)(const void*);
  uint32_t (*total_watt_hours_imported_callback)(const void*);
  uint32_t (*total_watt_hours_imported_phase_a_callback)(const void*);
  uint32_t (*total_watt_hours_imported_phase_b_callback)(const void*);
  uint32_t (*total_watt_hours_imported_phase_c_callback)(const void*);
  uint16_t (*tot_wh_sf_callback)(const void*);
  uint32_t (*total_va_hours_exported_callback)(const void*);
  uint32_t (*total_va_hours_exported_phase_a_callback)(const void*);
  uint32_t (*total_va_hours_exported_phase_b_callback)(const void*);
  uint32_t (*total_va_hours_exported_phase_c_callback)(const void*);
  uint32_t (*total_va_hours_imported_callback)(const void*);
  uint32_t (*total_va_hours_imported_phase_a_callback)(const void*);
  uint32_t (*total_va_hours_imported_phase_b_callback)(const void*);
  uint32_t (*total_va_hours_imported_phase_c_callback)(const void*);
  uint16_t (*tot_v_ah_sf_callback)(const void*);
  uint32_t (*total_var_hours_imported_q1_callback)(const void*);
  uint32_t (*total_v_ar_hours_imported_q1_phase_a_callback)(const void*);
  uint32_t (*total_v_ar_hours_imported_q1_phase_b_callback)(const void*);
  uint32_t (*total_v_ar_hours_imported_q1_phase_c_callback)(const void*);
  uint32_t (*total_v_ar_hours_imported_q2_callback)(const void*);
  uint32_t (*total_v_ar_hours_imported_q2_phase_a_callback)(const void*);
  uint32_t (*total_v_ar_hours_imported_q2_phase_b_callback)(const void*);
  uint32_t (*total_v_ar_hours_imported_q2_phase_c_callback)(const void*);
  uint32_t (*total_v_ar_hours_exported_q3_callback)(const void*);
  uint32_t (*total_v_ar_hours_exported_q3_phase_a_callback)(const void*);
  uint32_t (*total_v_ar_hours_exported_q3_phase_b_callback)(const void*);
  uint32_t (*total_v_ar_hours_exported_q3_phase_c_callback)(const void*);
  uint32_t (*total_v_ar_hours_exported_q4_callback)(const void*);
  uint32_t (*total_v_ar_hours_exported_q4_imported_phase_a_callback)(const void*);
  uint32_t (*total_v_ar_hours_exported_q4_imported_phase_b_callback)(const void*);
  uint32_t (*total_v_ar_hours_exported_q4_imported_phase_c_callback)(const void*);
  uint16_t (*tot_v_arh_sf_callback)(const void*);
  uint32_t (*events_callback)(const void*);
} Model202CallbackAdapter;

typedef struct Model202StatefulAdapter {
  int16_t amps;
  int16_t amps_phase_a;
  int16_t amps_phase_b;
  int16_t amps_phase_c;
  uint16_t a_sf;
  int16_t voltage_ln;
  int16_t phase_voltage_an;
  int16_t phase_voltage_bn;
  int16_t phase_voltage_cn;
  int16_t voltage_ll;
  int16_t phase_voltage_ab;
  int16_t phase_voltage_bc;
  int16_t phase_voltage_ca;
  uint16_t v_sf;
  int16_t hz;
  uint16_t hz_sf;
  int16_t watts;
  int16_t watts_phase_a;
  int16_t watts_phase_b;
  int16_t watts_phase_c;
  uint16_t w_sf;
  int16_t va;
  int16_t va_phase_a;
  int16_t va_phase_b;
  int16_t va_phase_c;
  uint16_t va_sf;
  int16_t var;
  int16_t var_phase_a;
  int16_t var_phase_b;
  int16_t var_phase_c;
  uint16_t var_sf;
  int16_t pf;
  int16_t pf_phase_a;
  int16_t pf_phase_b;
  int16_t pf_phase_c;
  uint16_t pf_sf;
  uint32_t total_watt_hours_exported;
  uint32_t total_watt_hours_exported_phase_a;
  uint32_t total_watt_hours_exported_phase_b;
  uint32_t total_watt_hours_exported_phase_c;
  uint32_t total_watt_hours_imported;
  uint32_t total_watt_hours_imported_phase_a;
  uint32_t total_watt_hours_imported_phase_b;
  uint32_t total_watt_hours_imported_phase_c;
  uint16_t tot_wh_sf;
  uint32_t total_va_hours_exported;
  uint32_t total_va_hours_exported_phase_a;
  uint32_t total_va_hours_exported_phase_b;
  uint32_t total_va_hours_exported_phase_c;
  uint32_t total_va_hours_imported;
  uint32_t total_va_hours_imported_phase_a;
  uint32_t total_va_hours_imported_phase_b;
  uint32_t total_va_hours_imported_phase_c;
  uint16_t tot_v_ah_sf;
  uint32_t total_var_hours_imported_q1;
  uint32_t total_v_ar_hours_imported_q1_phase_a;
  uint32_t total_v_ar_hours_imported_q1_phase_b;
  uint32_t total_v_ar_hours_imported_q1_phase_c;
  uint32_t total_v_ar_hours_imported_q2;
  uint32_t total_v_ar_hours_imported_q2_phase_a;
  uint32_t total_v_ar_hours_imported_q2_phase_b;
  uint32_t total_v_ar_hours_imported_q2_phase_c;
  uint32_t total_v_ar_hours_exported_q3;
  uint32_t total_v_ar_hours_exported_q3_phase_a;
  uint32_t total_v_ar_hours_exported_q3_phase_b;
  uint32_t total_v_ar_hours_exported_q3_phase_c;
  uint32_t total_v_ar_hours_exported_q4;
  uint32_t total_v_ar_hours_exported_q4_imported_phase_a;
  uint32_t total_v_ar_hours_exported_q4_imported_phase_b;
  uint32_t total_v_ar_hours_exported_q4_imported_phase_c;
  uint16_t tot_v_arh_sf;
  uint32_t events;
} Model202StatefulAdapter;

typedef struct Model203CallbackAdapter {
  void *context;
  int16_t (*amps_callback)(const void*);
  int16_t (*amps_phase_a_callback)(const void*);
  int16_t (*amps_phase_b_callback)(const void*);
  int16_t (*amps_phase_c_callback)(const void*);
  uint16_t (*a_sf_callback)(const void*);
  int16_t (*voltage_ln_callback)(const void*);
  int16_t (*phase_voltage_an_callback)(const void*);
  int16_t (*phase_voltage_bn_callback)(const void*);
  int16_t (*phase_voltage_cn_callback)(const void*);
  int16_t (*voltage_ll_callback)(const void*);
  int16_t (*phase_voltage_ab_callback)(const void*);
  int16_t (*phase_voltage_bc_callback)(const void*);
  int16_t (*phase_voltage_ca_callback)(const void*);
  uint16_t (*v_sf_callback)(const void*);
  int16_t (*hz_callback)(const void*);
  uint16_t (*hz_sf_callback)(const void*);
  int16_t (*watts_callback)(const void*);
  int16_t (*watts_phase_a_callback)(const void*);
  int16_t (*watts_phase_b_callback)(const void*);
  int16_t (*watts_phase_c_callback)(const void*);
  uint16_t (*w_sf_callback)(const void*);
  int16_t (*va_callback)(const void*);
  int16_t (*va_phase_a_callback)(const void*);
  int16_t (*va_phase_b_callback)(const void*);
  int16_t (*va_phase_c_callback)(const void*);
  uint16_t (*va_sf_callback)(const void*);
  int16_t (*var_callback)(const void*);
  int16_t (*var_phase_a_callback)(const void*);
  int16_t (*var_phase_b_callback)(const void*);
  int16_t (*var_phase_c_callback)(const void*);
  uint16_t (*var_sf_callback)(const void*);
  int16_t (*pf_callback)(const void*);
  int16_t (*pf_phase_a_callback)(const void*);
  int16_t (*pf_phase_b_callback)(const void*);
  int16_t (*pf_phase_c_callback)(const void*);
  uint16_t (*pf_sf_callback)(const void*);
  uint32_t (*total_watt_hours_exported_callback)(const void*);
  uint32_t (*total_watt_hours_exported_phase_a_callback)(const void*);
  uint32_t (*total_watt_hours_exported_phase_b_callback)(const void*);
  uint32_t (*total_watt_hours_exported_phase_c_callback)(const void*);
  uint32_t (*total_watt_hours_imported_callback)(const void*);
  uint32_t (*total_watt_hours_imported_phase_a_callback)(const void*);
  uint32_t (*total_watt_hours_imported_phase_b_callback)(const void*);
  uint32_t (*total_watt_hours_imported_phase_c_callback)(const void*);
  uint16_t (*tot_wh_sf_callback)(const void*);
  uint32_t (*total_va_hours_exported_callback)(const void*);
  uint32_t (*total_va_hours_exported_phase_a_callback)(const void*);
  uint32_t (*total_va_hours_exported_phase_b_callback)(const void*);
  uint32_t (*total_va_hours_exported_phase_c_callback)(const void*);
  uint32_t (*total_va_hours_imported_callback)(const void*);
  uint32_t (*total_va_hours_imported_phase_a_callback)(const void*);
  uint32_t (*total_va_hours_imported_phase_b_callback)(const void*);
  uint32_t (*total_va_hours_imported_phase_c_callback)(const void*);
  uint16_t (*tot_v_ah_sf_callback)(const void*);
  uint32_t (*total_var_hours_imported_q1_callback)(const void*);
  uint32_t (*total_v_ar_hours_imported_q1_phase_a_callback)(const void*);
  uint32_t (*total_v_ar_hours_imported_q1_phase_b_callback)(const void*);
  uint32_t (*total_v_ar_hours_imported_q1_phase_c_callback)(const void*);
  uint32_t (*total_v_ar_hours_imported_q2_callback)(const void*);
  uint32_t (*total_v_ar_hours_imported_q2_phase_a_callback)(const void*);
  uint32_t (*total_v_ar_hours_imported_q2_phase_b_callback)(const void*);
  uint32_t (*total_v_ar_hours_imported_q2_phase_c_callback)(const void*);
  uint32_t (*total_v_ar_hours_exported_q3_callback)(const void*);
  uint32_t (*total_v_ar_hours_exported_q3_phase_a_callback)(const void*);
  uint32_t (*total_v_ar_hours_exported_q3_phase_b_callback)(const void*);
  uint32_t (*total_v_ar_hours_exported_q3_phase_c_callback)(const void*);
  uint32_t (*total_v_ar_hours_exported_q4_callback)(const void*);
  uint32_t (*total_v_ar_hours_exported_q4_imported_phase_a_callback)(const void*);
  uint32_t (*total_v_ar_hours_exported_q4_imported_phase_b_callback)(const void*);
  uint32_t (*total_v_ar_hours_exported_q4_imported_phase_c_callback)(const void*);
  uint16_t (*tot_v_arh_sf_callback)(const void*);
  uint32_t (*events_callback)(const void*);
} Model203CallbackAdapter;

typedef struct Model203StatefulAdapter {
  int16_t amps;
  int16_t amps_phase_a;
  int16_t amps_phase_b;
  int16_t amps_phase_c;
  uint16_t a_sf;
  int16_t voltage_ln;
  int16_t phase_voltage_an;
  int16_t phase_voltage_bn;
  int16_t phase_voltage_cn;
  int16_t voltage_ll;
  int16_t phase_voltage_ab;
  int16_t phase_voltage_bc;
  int16_t phase_voltage_ca;
  uint16_t v_sf;
  int16_t hz;
  uint16_t hz_sf;
  int16_t watts;
  int16_t watts_phase_a;
  int16_t watts_phase_b;
  int16_t watts_phase_c;
  uint16_t w_sf;
  int16_t va;
  int16_t va_phase_a;
  int16_t va_phase_b;
  int16_t va_phase_c;
  uint16_t va_sf;
  int16_t var;
  int16_t var_phase_a;
  int16_t var_phase_b;
  int16_t var_phase_c;
  uint16_t var_sf;
  int16_t pf;
  int16_t pf_phase_a;
  int16_t pf_phase_b;
  int16_t pf_phase_c;
  uint16_t pf_sf;
  uint32_t total_watt_hours_exported;
  uint32_t total_watt_hours_exported_phase_a;
  uint32_t total_watt_hours_exported_phase_b;
  uint32_t total_watt_hours_exported_phase_c;
  uint32_t total_watt_hours_imported;
  uint32_t total_watt_hours_imported_phase_a;
  uint32_t total_watt_hours_imported_phase_b;
  uint32_t total_watt_hours_imported_phase_c;
  uint16_t tot_wh_sf;
  uint32_t total_va_hours_exported;
  uint32_t total_va_hours_exported_phase_a;
  uint32_t total_va_hours_exported_phase_b;
  uint32_t total_va_hours_exported_phase_c;
  uint32_t total_va_hours_imported;
  uint32_t total_va_hours_imported_phase_a;
  uint32_t total_va_hours_imported_phase_b;
  uint32_t total_va_hours_imported_phase_c;
  uint16_t tot_v_ah_sf;
  uint32_t total_var_hours_imported_q1;
  uint32_t total_v_ar_hours_imported_q1_phase_a;
  uint32_t total_v_ar_hours_imported_q1_phase_b;
  uint32_t total_v_ar_hours_imported_q1_phase_c;
  uint32_t total_v_ar_hours_imported_q2;
  uint32_t total_v_ar_hours_imported_q2_phase_a;
  uint32_t total_v_ar_hours_imported_q2_phase_b;
  uint32_t total_v_ar_hours_imported_q2_phase_c;
  uint32_t total_v_ar_hours_exported_q3;
  uint32_t total_v_ar_hours_exported_q3_phase_a;
  uint32_t total_v_ar_hours_exported_q3_phase_b;
  uint32_t total_v_ar_hours_exported_q3_phase_c;
  uint32_t total_v_ar_hours_exported_q4;
  uint32_t total_v_ar_hours_exported_q4_imported_phase_a;
  uint32_t total_v_ar_hours_exported_q4_imported_phase_b;
  uint32_t total_v_ar_hours_exported_q4_imported_phase_c;
  uint16_t tot_v_arh_sf;
  uint32_t events;
} Model203StatefulAdapter;

typedef struct Model204CallbackAdapter {
  void *context;
  int16_t (*amps_callback)(const void*);
  int16_t (*amps_phase_a_callback)(const void*);
  int16_t (*amps_phase_b_callback)(const void*);
  int16_t (*amps_phase_c_callback)(const void*);
  uint16_t (*a_sf_callback)(const void*);
  int16_t (*voltage_ln_callback)(const void*);
  int16_t (*phase_voltage_an_callback)(const void*);
  int16_t (*phase_voltage_bn_callback)(const void*);
  int16_t (*phase_voltage_cn_callback)(const void*);
  int16_t (*voltage_ll_callback)(const void*);
  int16_t (*phase_voltage_ab_callback)(const void*);
  int16_t (*phase_voltage_bc_callback)(const void*);
  int16_t (*phase_voltage_ca_callback)(const void*);
  uint16_t (*v_sf_callback)(const void*);
  int16_t (*hz_callback)(const void*);
  uint16_t (*hz_sf_callback)(const void*);
  int16_t (*watts_callback)(const void*);
  int16_t (*watts_phase_a_callback)(const void*);
  int16_t (*watts_phase_b_callback)(const void*);
  int16_t (*watts_phase_c_callback)(const void*);
  uint16_t (*w_sf_callback)(const void*);
  int16_t (*va_callback)(const void*);
  int16_t (*va_phase_a_callback)(const void*);
  int16_t (*va_phase_b_callback)(const void*);
  int16_t (*va_phase_c_callback)(const void*);
  uint16_t (*va_sf_callback)(const void*);
  int16_t (*var_callback)(const void*);
  int16_t (*var_phase_a_callback)(const void*);
  int16_t (*var_phase_b_callback)(const void*);
  int16_t (*var_phase_c_callback)(const void*);
  uint16_t (*var_sf_callback)(const void*);
  int16_t (*pf_callback)(const void*);
  int16_t (*pf_phase_a_callback)(const void*);
  int16_t (*pf_phase_b_callback)(const void*);
  int16_t (*pf_phase_c_callback)(const void*);
  uint16_t (*pf_sf_callback)(const void*);
  uint32_t (*total_watt_hours_exported_callback)(const void*);
  uint32_t (*total_watt_hours_exported_phase_a_callback)(const void*);
  uint32_t (*total_watt_hours_exported_phase_b_callback)(const void*);
  uint32_t (*total_watt_hours_exported_phase_c_callback)(const void*);
  uint32_t (*total_watt_hours_imported_callback)(const void*);
  uint32_t (*total_watt_hours_imported_phase_a_callback)(const void*);
  uint32_t (*total_watt_hours_imported_phase_b_callback)(const void*);
  uint32_t (*total_watt_hours_imported_phase_c_callback)(const void*);
  uint16_t (*tot_wh_sf_callback)(const void*);
  uint32_t (*total_va_hours_exported_callback)(const void*);
  uint32_t (*total_va_hours_exported_phase_a_callback)(const void*);
  uint32_t (*total_va_hours_exported_phase_b_callback)(const void*);
  uint32_t (*total_va_hours_exported_phase_c_callback)(const void*);
  uint32_t (*total_va_hours_imported_callback)(const void*);
  uint32_t (*total_va_hours_imported_phase_a_callback)(const void*);
  uint32_t (*total_va_hours_imported_phase_b_callback)(const void*);
  uint32_t (*total_va_hours_imported_phase_c_callback)(const void*);
  uint16_t (*tot_v_ah_sf_callback)(const void*);
  uint32_t (*total_var_hours_imported_q1_callback)(const void*);
  uint32_t (*total_v_ar_hours_imported_q1_phase_a_callback)(const void*);
  uint32_t (*total_v_ar_hours_imported_q1_phase_b_callback)(const void*);
  uint32_t (*total_v_ar_hours_imported_q1_phase_c_callback)(const void*);
  uint32_t (*total_v_ar_hours_imported_q2_callback)(const void*);
  uint32_t (*total_v_ar_hours_imported_q2_phase_a_callback)(const void*);
  uint32_t (*total_v_ar_hours_imported_q2_phase_b_callback)(const void*);
  uint32_t (*total_v_ar_hours_imported_q2_phase_c_callback)(const void*);
  uint32_t (*total_v_ar_hours_exported_q3_callback)(const void*);
  uint32_t (*total_v_ar_hours_exported_q3_phase_a_callback)(const void*);
  uint32_t (*total_v_ar_hours_exported_q3_phase_b_callback)(const void*);
  uint32_t (*total_v_ar_hours_exported_q3_phase_c_callback)(const void*);
  uint32_t (*total_v_ar_hours_exported_q4_callback)(const void*);
  uint32_t (*total_v_ar_hours_exported_q4_imported_phase_a_callback)(const void*);
  uint32_t (*total_v_ar_hours_exported_q4_imported_phase_b_callback)(const void*);
  uint32_t (*total_v_ar_hours_exported_q4_imported_phase_c_callback)(const void*);
  uint16_t (*tot_v_arh_sf_callback)(const void*);
  uint32_t (*events_callback)(const void*);
} Model204CallbackAdapter;

typedef struct Model204StatefulAdapter {
  int16_t amps;
  int16_t amps_phase_a;
  int16_t amps_phase_b;
  int16_t amps_phase_c;
  uint16_t a_sf;
  int16_t voltage_ln;
  int16_t phase_voltage_an;
  int16_t phase_voltage_bn;
  int16_t phase_voltage_cn;
  int16_t voltage_ll;
  int16_t phase_voltage_ab;
  int16_t phase_voltage_bc;
  int16_t phase_voltage_ca;
  uint16_t v_sf;
  int16_t hz;
  uint16_t hz_sf;
  int16_t watts;
  int16_t watts_phase_a;
  int16_t watts_phase_b;
  int16_t watts_phase_c;
  uint16_t w_sf;
  int16_t va;
  int16_t va_phase_a;
  int16_t va_phase_b;
  int16_t va_phase_c;
  uint16_t va_sf;
  int16_t var;
  int16_t var_phase_a;
  int16_t var_phase_b;
  int16_t var_phase_c;
  uint16_t var_sf;
  int16_t pf;
  int16_t pf_phase_a;
  int16_t pf_phase_b;
  int16_t pf_phase_c;
  uint16_t pf_sf;
  uint32_t total_watt_hours_exported;
  uint32_t total_watt_hours_exported_phase_a;
  uint32_t total_watt_hours_exported_phase_b;
  uint32_t total_watt_hours_exported_phase_c;
  uint32_t total_watt_hours_imported;
  uint32_t total_watt_hours_imported_phase_a;
  uint32_t total_watt_hours_imported_phase_b;
  uint32_t total_watt_hours_imported_phase_c;
  uint16_t tot_wh_sf;
  uint32_t total_va_hours_exported;
  uint32_t total_va_hours_exported_phase_a;
  uint32_t total_va_hours_exported_phase_b;
  uint32_t total_va_hours_exported_phase_c;
  uint32_t total_va_hours_imported;
  uint32_t total_va_hours_imported_phase_a;
  uint32_t total_va_hours_imported_phase_b;
  uint32_t total_va_hours_imported_phase_c;
  uint16_t tot_v_ah_sf;
  uint32_t total_var_hours_imported_q1;
  uint32_t total_v_ar_hours_imported_q1_phase_a;
  uint32_t total_v_ar_hours_imported_q1_phase_b;
  uint32_t total_v_ar_hours_imported_q1_phase_c;
  uint32_t total_v_ar_hours_imported_q2;
  uint32_t total_v_ar_hours_imported_q2_phase_a;
  uint32_t total_v_ar_hours_imported_q2_phase_b;
  uint32_t total_v_ar_hours_imported_q2_phase_c;
  uint32_t total_v_ar_hours_exported_q3;
  uint32_t total_v_ar_hours_exported_q3_phase_a;
  uint32_t total_v_ar_hours_exported_q3_phase_b;
  uint32_t total_v_ar_hours_exported_q3_phase_c;
  uint32_t total_v_ar_hours_exported_q4;
  uint32_t total_v_ar_hours_exported_q4_imported_phase_a;
  uint32_t total_v_ar_hours_exported_q4_imported_phase_b;
  uint32_t total_v_ar_hours_exported_q4_imported_phase_c;
  uint16_t tot_v_arh_sf;
  uint32_t events;
} Model204StatefulAdapter;

typedef struct Model211CallbackAdapter {
  void *context;
  float (*amps_callback)(const void*);
  float (*amps_phase_a_callback)(const void*);
  float (*amps_phase_b_callback)(const void*);
  float (*amps_phase_c_callback)(const void*);
  float (*voltage_ln_callback)(const void*);
  float (*phase_voltage_an_callback)(const void*);
  float (*phase_voltage_bn_callback)(const void*);
  float (*phase_voltage_cn_callback)(const void*);
  float (*voltage_ll_callback)(const void*);
  float (*phase_voltage_ab_callback)(const void*);
  float (*phase_voltage_bc_callback)(const void*);
  float (*phase_voltage_ca_callback)(const void*);
  float (*hz_callback)(const void*);
  float (*watts_callback)(const void*);
  float (*watts_phase_a_callback)(const void*);
  float (*watts_phase_b_callback)(const void*);
  float (*watts_phase_c_callback)(const void*);
  float (*va_callback)(const void*);
  float (*va_phase_a_callback)(const void*);
  float (*va_phase_b_callback)(const void*);
  float (*va_phase_c_callback)(const void*);
  float (*var_callback)(const void*);
  float (*var_phase_a_callback)(const void*);
  float (*var_phase_b_callback)(const void*);
  float (*var_phase_c_callback)(const void*);
  float (*pf_callback)(const void*);
  float (*pf_phase_a_callback)(const void*);
  float (*pf_phase_b_callback)(const void*);
  float (*pf_phase_c_callback)(const void*);
  float (*total_watt_hours_exported_callback)(const void*);
  float (*total_watt_hours_exported_phase_a_callback)(const void*);
  float (*total_watt_hours_exported_phase_b_callback)(const void*);
  float (*total_watt_hours_exported_phase_c_callback)(const void*);
  float (*total_watt_hours_imported_callback)(const void*);
  float (*total_watt_hours_imported_phase_a_callback)(const void*);
  float (*total_watt_hours_imported_phase_b_callback)(const void*);
  float (*total_watt_hours_imported_phase_c_callback)(const void*);
  float (*total_va_hours_exported_callback)(const void*);
  float (*total_va_hours_exported_phase_a_callback)(const void*);
  float (*total_va_hours_exported_phase_b_callback)(const void*);
  float (*total_va_hours_exported_phase_c_callback)(const void*);
  float (*total_va_hours_imported_callback)(const void*);
  float (*total_va_hours_imported_phase_a_callback)(const void*);
  float (*total_va_hours_imported_phase_b_callback)(const void*);
  float (*total_va_hours_imported_phase_c_callback)(const void*);
  float (*total_var_hours_imported_q1_callback)(const void*);
  float (*total_v_ar_hours_imported_q1_phase_a_callback)(const void*);
  float (*total_v_ar_hours_imported_q1_phase_b_callback)(const void*);
  float (*total_v_ar_hours_imported_q1_phase_c_callback)(const void*);
  float (*total_v_ar_hours_imported_q2_callback)(const void*);
  float (*total_v_ar_hours_imported_q2_phase_a_callback)(const void*);
  float (*total_v_ar_hours_imported_q2_phase_b_callback)(const void*);
  float (*total_v_ar_hours_imported_q2_phase_c_callback)(const void*);
  float (*total_v_ar_hours_exported_q3_callback)(const void*);
  float (*total_v_ar_hours_exported_q3_phase_a_callback)(const void*);
  float (*total_v_ar_hours_exported_q3_phase_b_callback)(const void*);
  float (*total_v_ar_hours_exported_q3_phase_c_callback)(const void*);
  float (*total_v_ar_hours_exported_q4_callback)(const void*);
  float (*total_v_ar_hours_exported_q4_imported_phase_a_callback)(const void*);
  float (*total_v_ar_hours_exported_q4_imported_phase_b_callback)(const void*);
  float (*total_v_ar_hours_exported_q4_imported_phase_c_callback)(const void*);
  uint32_t (*events_callback)(const void*);
} Model211CallbackAdapter;

typedef struct Model211StatefulAdapter {
  float amps;
  float amps_phase_a;
  float amps_phase_b;
  float amps_phase_c;
  float voltage_ln;
  float phase_voltage_an;
  float phase_voltage_bn;
  float phase_voltage_cn;
  float voltage_ll;
  float phase_voltage_ab;
  float phase_voltage_bc;
  float phase_voltage_ca;
  float hz;
  float watts;
  float watts_phase_a;
  float watts_phase_b;
  float watts_phase_c;
  float va;
  float va_phase_a;
  float va_phase_b;
  float va_phase_c;
  float var;
  float var_phase_a;
  float var_phase_b;
  float var_phase_c;
  float pf;
  float pf_phase_a;
  float pf_phase_b;
  float pf_phase_c;
  float total_watt_hours_exported;
  float total_watt_hours_exported_phase_a;
  float total_watt_hours_exported_phase_b;
  float total_watt_hours_exported_phase_c;
  float total_watt_hours_imported;
  float total_watt_hours_imported_phase_a;
  float total_watt_hours_imported_phase_b;
  float total_watt_hours_imported_phase_c;
  float total_va_hours_exported;
  float total_va_hours_exported_phase_a;
  float total_va_hours_exported_phase_b;
  float total_va_hours_exported_phase_c;
  float total_va_hours_imported;
  float total_va_hours_imported_phase_a;
  float total_va_hours_imported_phase_b;
  float total_va_hours_imported_phase_c;
  float total_var_hours_imported_q1;
  float total_v_ar_hours_imported_q1_phase_a;
  float total_v_ar_hours_imported_q1_phase_b;
  float total_v_ar_hours_imported_q1_phase_c;
  float total_v_ar_hours_imported_q2;
  float total_v_ar_hours_imported_q2_phase_a;
  float total_v_ar_hours_imported_q2_phase_b;
  float total_v_ar_hours_imported_q2_phase_c;
  float total_v_ar_hours_exported_q3;
  float total_v_ar_hours_exported_q3_phase_a;
  float total_v_ar_hours_exported_q3_phase_b;
  float total_v_ar_hours_exported_q3_phase_c;
  float total_v_ar_hours_exported_q4;
  float total_v_ar_hours_exported_q4_imported_phase_a;
  float total_v_ar_hours_exported_q4_imported_phase_b;
  float total_v_ar_hours_exported_q4_imported_phase_c;
  uint32_t events;
} Model211StatefulAdapter;

typedef struct Model212CallbackAdapter {
  void *context;
  float (*amps_callback)(const void*);
  float (*amps_phase_a_callback)(const void*);
  float (*amps_phase_b_callback)(const void*);
  float (*amps_phase_c_callback)(const void*);
  float (*voltage_ln_callback)(const void*);
  float (*phase_voltage_an_callback)(const void*);
  float (*phase_voltage_bn_callback)(const void*);
  float (*phase_voltage_cn_callback)(const void*);
  float (*voltage_ll_callback)(const void*);
  float (*phase_voltage_ab_callback)(const void*);
  float (*phase_voltage_bc_callback)(const void*);
  float (*phase_voltage_ca_callback)(const void*);
  float (*hz_callback)(const void*);
  float (*watts_callback)(const void*);
  float (*watts_phase_a_callback)(const void*);
  float (*watts_phase_b_callback)(const void*);
  float (*watts_phase_c_callback)(const void*);
  float (*va_callback)(const void*);
  float (*va_phase_a_callback)(const void*);
  float (*va_phase_b_callback)(const void*);
  float (*va_phase_c_callback)(const void*);
  float (*var_callback)(const void*);
  float (*var_phase_a_callback)(const void*);
  float (*var_phase_b_callback)(const void*);
  float (*var_phase_c_callback)(const void*);
  float (*pf_callback)(const void*);
  float (*pf_phase_a_callback)(const void*);
  float (*pf_phase_b_callback)(const void*);
  float (*pf_phase_c_callback)(const void*);
  float (*total_watt_hours_exported_callback)(const void*);
  float (*total_watt_hours_exported_phase_a_callback)(const void*);
  float (*total_watt_hours_exported_phase_b_callback)(const void*);
  float (*total_watt_hours_exported_phase_c_callback)(const void*);
  float (*total_watt_hours_imported_callback)(const void*);
  float (*total_watt_hours_imported_phase_a_callback)(const void*);
  float (*total_watt_hours_imported_phase_b_callback)(const void*);
  float (*total_watt_hours_imported_phase_c_callback)(const void*);
  float (*total_va_hours_exported_callback)(const void*);
  float (*total_va_hours_exported_phase_a_callback)(const void*);
  float (*total_va_hours_exported_phase_b_callback)(const void*);
  float (*total_va_hours_exported_phase_c_callback)(const void*);
  float (*total_va_hours_imported_callback)(const void*);
  float (*total_va_hours_imported_phase_a_callback)(const void*);
  float (*total_va_hours_imported_phase_b_callback)(const void*);
  float (*total_va_hours_imported_phase_c_callback)(const void*);
  float (*total_var_hours_imported_q1_callback)(const void*);
  float (*total_v_ar_hours_imported_q1_phase_a_callback)(const void*);
  float (*total_v_ar_hours_imported_q1_phase_b_callback)(const void*);
  float (*total_v_ar_hours_imported_q1_phase_c_callback)(const void*);
  float (*total_v_ar_hours_imported_q2_callback)(const void*);
  float (*total_v_ar_hours_imported_q2_phase_a_callback)(const void*);
  float (*total_v_ar_hours_imported_q2_phase_b_callback)(const void*);
  float (*total_v_ar_hours_imported_q2_phase_c_callback)(const void*);
  float (*total_v_ar_hours_exported_q3_callback)(const void*);
  float (*total_v_ar_hours_exported_q3_phase_a_callback)(const void*);
  float (*total_v_ar_hours_exported_q3_phase_b_callback)(const void*);
  float (*total_v_ar_hours_exported_q3_phase_c_callback)(const void*);
  float (*total_v_ar_hours_exported_q4_callback)(const void*);
  float (*total_v_ar_hours_exported_q4_imported_phase_a_callback)(const void*);
  float (*total_v_ar_hours_exported_q4_imported_phase_b_callback)(const void*);
  float (*total_v_ar_hours_exported_q4_imported_phase_c_callback)(const void*);
  uint32_t (*events_callback)(const void*);
} Model212CallbackAdapter;

typedef struct Model212StatefulAdapter {
  float amps;
  float amps_phase_a;
  float amps_phase_b;
  float amps_phase_c;
  float voltage_ln;
  float phase_voltage_an;
  float phase_voltage_bn;
  float phase_voltage_cn;
  float voltage_ll;
  float phase_voltage_ab;
  float phase_voltage_bc;
  float phase_voltage_ca;
  float hz;
  float watts;
  float watts_phase_a;
  float watts_phase_b;
  float watts_phase_c;
  float va;
  float va_phase_a;
  float va_phase_b;
  float va_phase_c;
  float var;
  float var_phase_a;
  float var_phase_b;
  float var_phase_c;
  float pf;
  float pf_phase_a;
  float pf_phase_b;
  float pf_phase_c;
  float total_watt_hours_exported;
  float total_watt_hours_exported_phase_a;
  float total_watt_hours_exported_phase_b;
  float total_watt_hours_exported_phase_c;
  float total_watt_hours_imported;
  float total_watt_hours_imported_phase_a;
  float total_watt_hours_imported_phase_b;
  float total_watt_hours_imported_phase_c;
  float total_va_hours_exported;
  float total_va_hours_exported_phase_a;
  float total_va_hours_exported_phase_b;
  float total_va_hours_exported_phase_c;
  float total_va_hours_imported;
  float total_va_hours_imported_phase_a;
  float total_va_hours_imported_phase_b;
  float total_va_hours_imported_phase_c;
  float total_var_hours_imported_q1;
  float total_v_ar_hours_imported_q1_phase_a;
  float total_v_ar_hours_imported_q1_phase_b;
  float total_v_ar_hours_imported_q1_phase_c;
  float total_v_ar_hours_imported_q2;
  float total_v_ar_hours_imported_q2_phase_a;
  float total_v_ar_hours_imported_q2_phase_b;
  float total_v_ar_hours_imported_q2_phase_c;
  float total_v_ar_hours_exported_q3;
  float total_v_ar_hours_exported_q3_phase_a;
  float total_v_ar_hours_exported_q3_phase_b;
  float total_v_ar_hours_exported_q3_phase_c;
  float total_v_ar_hours_exported_q4;
  float total_v_ar_hours_exported_q4_imported_phase_a;
  float total_v_ar_hours_exported_q4_imported_phase_b;
  float total_v_ar_hours_exported_q4_imported_phase_c;
  uint32_t events;
} Model212StatefulAdapter;

typedef struct Model213CallbackAdapter {
  void *context;
  float (*amps_callback)(const void*);
  float (*amps_phase_a_callback)(const void*);
  float (*amps_phase_b_callback)(const void*);
  float (*amps_phase_c_callback)(const void*);
  float (*voltage_ln_callback)(const void*);
  float (*phase_voltage_an_callback)(const void*);
  float (*phase_voltage_bn_callback)(const void*);
  float (*phase_voltage_cn_callback)(const void*);
  float (*voltage_ll_callback)(const void*);
  float (*phase_voltage_ab_callback)(const void*);
  float (*phase_voltage_bc_callback)(const void*);
  float (*phase_voltage_ca_callback)(const void*);
  float (*hz_callback)(const void*);
  float (*watts_callback)(const void*);
  float (*watts_phase_a_callback)(const void*);
  float (*watts_phase_b_callback)(const void*);
  float (*watts_phase_c_callback)(const void*);
  float (*va_callback)(const void*);
  float (*va_phase_a_callback)(const void*);
  float (*va_phase_b_callback)(const void*);
  float (*va_phase_c_callback)(const void*);
  float (*var_callback)(const void*);
  float (*var_phase_a_callback)(const void*);
  float (*var_phase_b_callback)(const void*);
  float (*var_phase_c_callback)(const void*);
  float (*pf_callback)(const void*);
  float (*pf_phase_a_callback)(const void*);
  float (*pf_phase_b_callback)(const void*);
  float (*pf_phase_c_callback)(const void*);
  float (*total_watt_hours_exported_callback)(const void*);
  float (*total_watt_hours_exported_phase_a_callback)(const void*);
  float (*total_watt_hours_exported_phase_b_callback)(const void*);
  float (*total_watt_hours_exported_phase_c_callback)(const void*);
  float (*total_watt_hours_imported_callback)(const void*);
  float (*total_watt_hours_imported_phase_a_callback)(const void*);
  float (*total_watt_hours_imported_phase_b_callback)(const void*);
  float (*total_watt_hours_imported_phase_c_callback)(const void*);
  float (*total_va_hours_exported_callback)(const void*);
  float (*total_va_hours_exported_phase_a_callback)(const void*);
  float (*total_va_hours_exported_phase_b_callback)(const void*);
  float (*total_va_hours_exported_phase_c_callback)(const void*);
  float (*total_va_hours_imported_callback)(const void*);
  float (*total_va_hours_imported_phase_a_callback)(const void*);
  float (*total_va_hours_imported_phase_b_callback)(const void*);
  float (*total_va_hours_imported_phase_c_callback)(const void*);
  float (*total_var_hours_imported_q1_callback)(const void*);
  float (*total_v_ar_hours_imported_q1_phase_a_callback)(const void*);
  float (*total_v_ar_hours_imported_q1_phase_b_callback)(const void*);
  float (*total_v_ar_hours_imported_q1_phase_c_callback)(const void*);
  float (*total_v_ar_hours_imported_q2_callback)(const void*);
  float (*total_v_ar_hours_imported_q2_phase_a_callback)(const void*);
  float (*total_v_ar_hours_imported_q2_phase_b_callback)(const void*);
  float (*total_v_ar_hours_imported_q2_phase_c_callback)(const void*);
  float (*total_v_ar_hours_exported_q3_callback)(const void*);
  float (*total_v_ar_hours_exported_q3_phase_a_callback)(const void*);
  float (*total_v_ar_hours_exported_q3_phase_b_callback)(const void*);
  float (*total_v_ar_hours_exported_q3_phase_c_callback)(const void*);
  float (*total_v_ar_hours_exported_q4_callback)(const void*);
  float (*total_v_ar_hours_exported_q4_imported_phase_a_callback)(const void*);
  float (*total_v_ar_hours_exported_q4_imported_phase_b_callback)(const void*);
  float (*total_v_ar_hours_exported_q4_imported_phase_c_callback)(const void*);
  uint32_t (*events_callback)(const void*);
} Model213CallbackAdapter;

typedef struct Model213StatefulAdapter {
  float amps;
  float amps_phase_a;
  float amps_phase_b;
  float amps_phase_c;
  float voltage_ln;
  float phase_voltage_an;
  float phase_voltage_bn;
  float phase_voltage_cn;
  float voltage_ll;
  float phase_voltage_ab;
  float phase_voltage_bc;
  float phase_voltage_ca;
  float hz;
  float watts;
  float watts_phase_a;
  float watts_phase_b;
  float watts_phase_c;
  float va;
  float va_phase_a;
  float va_phase_b;
  float va_phase_c;
  float var;
  float var_phase_a;
  float var_phase_b;
  float var_phase_c;
  float pf;
  float pf_phase_a;
  float pf_phase_b;
  float pf_phase_c;
  float total_watt_hours_exported;
  float total_watt_hours_exported_phase_a;
  float total_watt_hours_exported_phase_b;
  float total_watt_hours_exported_phase_c;
  float total_watt_hours_imported;
  float total_watt_hours_imported_phase_a;
  float total_watt_hours_imported_phase_b;
  float total_watt_hours_imported_phase_c;
  float total_va_hours_exported;
  float total_va_hours_exported_phase_a;
  float total_va_hours_exported_phase_b;
  float total_va_hours_exported_phase_c;
  float total_va_hours_imported;
  float total_va_hours_imported_phase_a;
  float total_va_hours_imported_phase_b;
  float total_va_hours_imported_phase_c;
  float total_var_hours_imported_q1;
  float total_v_ar_hours_imported_q1_phase_a;
  float total_v_ar_hours_imported_q1_phase_b;
  float total_v_ar_hours_imported_q1_phase_c;
  float total_v_ar_hours_imported_q2;
  float total_v_ar_hours_imported_q2_phase_a;
  float total_v_ar_hours_imported_q2_phase_b;
  float total_v_ar_hours_imported_q2_phase_c;
  float total_v_ar_hours_exported_q3;
  float total_v_ar_hours_exported_q3_phase_a;
  float total_v_ar_hours_exported_q3_phase_b;
  float total_v_ar_hours_exported_q3_phase_c;
  float total_v_ar_hours_exported_q4;
  float total_v_ar_hours_exported_q4_imported_phase_a;
  float total_v_ar_hours_exported_q4_imported_phase_b;
  float total_v_ar_hours_exported_q4_imported_phase_c;
  uint32_t events;
} Model213StatefulAdapter;

typedef struct Model214CallbackAdapter {
  void *context;
  float (*amps_callback)(const void*);
  float (*amps_phase_a_callback)(const void*);
  float (*amps_phase_b_callback)(const void*);
  float (*amps_phase_c_callback)(const void*);
  float (*voltage_ln_callback)(const void*);
  float (*phase_voltage_an_callback)(const void*);
  float (*phase_voltage_bn_callback)(const void*);
  float (*phase_voltage_cn_callback)(const void*);
  float (*voltage_ll_callback)(const void*);
  float (*phase_voltage_ab_callback)(const void*);
  float (*phase_voltage_bc_callback)(const void*);
  float (*phase_voltage_ca_callback)(const void*);
  float (*hz_callback)(const void*);
  float (*watts_callback)(const void*);
  float (*watts_phase_a_callback)(const void*);
  float (*watts_phase_b_callback)(const void*);
  float (*watts_phase_c_callback)(const void*);
  float (*va_callback)(const void*);
  float (*va_phase_a_callback)(const void*);
  float (*va_phase_b_callback)(const void*);
  float (*va_phase_c_callback)(const void*);
  float (*var_callback)(const void*);
  float (*var_phase_a_callback)(const void*);
  float (*var_phase_b_callback)(const void*);
  float (*var_phase_c_callback)(const void*);
  float (*pf_callback)(const void*);
  float (*pf_phase_a_callback)(const void*);
  float (*pf_phase_b_callback)(const void*);
  float (*pf_phase_c_callback)(const void*);
  float (*total_watt_hours_exported_callback)(const void*);
  float (*total_watt_hours_exported_phase_a_callback)(const void*);
  float (*total_watt_hours_exported_phase_b_callback)(const void*);
  float (*total_watt_hours_exported_phase_c_callback)(const void*);
  float (*total_watt_hours_imported_callback)(const void*);
  float (*total_watt_hours_imported_phase_a_callback)(const void*);
  float (*total_watt_hours_imported_phase_b_callback)(const void*);
  float (*total_watt_hours_imported_phase_c_callback)(const void*);
  float (*total_va_hours_exported_callback)(const void*);
  float (*total_va_hours_exported_phase_a_callback)(const void*);
  float (*total_va_hours_exported_phase_b_callback)(const void*);
  float (*total_va_hours_exported_phase_c_callback)(const void*);
  float (*total_va_hours_imported_callback)(const void*);
  float (*total_va_hours_imported_phase_a_callback)(const void*);
  float (*total_va_hours_imported_phase_b_callback)(const void*);
  float (*total_va_hours_imported_phase_c_callback)(const void*);
  float (*total_var_hours_imported_q1_callback)(const void*);
  float (*total_v_ar_hours_imported_q1_phase_a_callback)(const void*);
  float (*total_v_ar_hours_imported_q1_phase_b_callback)(const void*);
  float (*total_v_ar_hours_imported_q1_phase_c_callback)(const void*);
  float (*total_v_ar_hours_imported_q2_callback)(const void*);
  float (*total_v_ar_hours_imported_q2_phase_a_callback)(const void*);
  float (*total_v_ar_hours_imported_q2_phase_b_callback)(const void*);
  float (*total_v_ar_hours_imported_q2_phase_c_callback)(const void*);
  float (*total_v_ar_hours_exported_q3_callback)(const void*);
  float (*total_v_ar_hours_exported_q3_phase_a_callback)(const void*);
  float (*total_v_ar_hours_exported_q3_phase_b_callback)(const void*);
  float (*total_v_ar_hours_exported_q3_phase_c_callback)(const void*);
  float (*total_v_ar_hours_exported_q4_callback)(const void*);
  float (*total_v_ar_hours_exported_q4_imported_phase_a_callback)(const void*);
  float (*total_v_ar_hours_exported_q4_imported_phase_b_callback)(const void*);
  float (*total_v_ar_hours_exported_q4_imported_phase_c_callback)(const void*);
  uint32_t (*events_callback)(const void*);
} Model214CallbackAdapter;

typedef struct Model214StatefulAdapter {
  float amps;
  float amps_phase_a;
  float amps_phase_b;
  float amps_phase_c;
  float voltage_ln;
  float phase_voltage_an;
  float phase_voltage_bn;
  float phase_voltage_cn;
  float voltage_ll;
  float phase_voltage_ab;
  float phase_voltage_bc;
  float phase_voltage_ca;
  float hz;
  float watts;
  float watts_phase_a;
  float watts_phase_b;
  float watts_phase_c;
  float va;
  float va_phase_a;
  float va_phase_b;
  float va_phase_c;
  float var;
  float var_phase_a;
  float var_phase_b;
  float var_phase_c;
  float pf;
  float pf_phase_a;
  float pf_phase_b;
  float pf_phase_c;
  float total_watt_hours_exported;
  float total_watt_hours_exported_phase_a;
  float total_watt_hours_exported_phase_b;
  float total_watt_hours_exported_phase_c;
  float total_watt_hours_imported;
  float total_watt_hours_imported_phase_a;
  float total_watt_hours_imported_phase_b;
  float total_watt_hours_imported_phase_c;
  float total_va_hours_exported;
  float total_va_hours_exported_phase_a;
  float total_va_hours_exported_phase_b;
  float total_va_hours_exported_phase_c;
  float total_va_hours_imported;
  float total_va_hours_imported_phase_a;
  float total_va_hours_imported_phase_b;
  float total_va_hours_imported_phase_c;
  float total_var_hours_imported_q1;
  float total_v_ar_hours_imported_q1_phase_a;
  float total_v_ar_hours_imported_q1_phase_b;
  float total_v_ar_hours_imported_q1_phase_c;
  float total_v_ar_hours_imported_q2;
  float total_v_ar_hours_imported_q2_phase_a;
  float total_v_ar_hours_imported_q2_phase_b;
  float total_v_ar_hours_imported_q2_phase_c;
  float total_v_ar_hours_exported_q3;
  float total_v_ar_hours_exported_q3_phase_a;
  float total_v_ar_hours_exported_q3_phase_b;
  float total_v_ar_hours_exported_q3_phase_c;
  float total_v_ar_hours_exported_q4;
  float total_v_ar_hours_exported_q4_imported_phase_a;
  float total_v_ar_hours_exported_q4_imported_phase_b;
  float total_v_ar_hours_exported_q4_imported_phase_c;
  uint32_t events;
} Model214StatefulAdapter;

typedef struct Model220CallbackAdapter {
  void *context;
  int16_t (*amps_callback)(const void*);
  uint16_t (*a_sf_callback)(const void*);
  int16_t (*voltage_callback)(const void*);
  uint16_t (*v_sf_callback)(const void*);
  int16_t (*hz_callback)(const void*);
  uint16_t (*hz_sf_callback)(const void*);
  int16_t (*watts_callback)(const void*);
  uint16_t (*w_sf_callback)(const void*);
  int16_t (*va_callback)(const void*);
  uint16_t (*va_sf_callback)(const void*);
  int16_t (*var_callback)(const void*);
  uint16_t (*var_sf_callback)(const void*);
  int16_t (*pf_callback)(const void*);
  uint16_t (*pf_sf_callback)(const void*);
  uint32_t (*total_watt_hours_exported_callback)(const void*);
  uint32_t (*total_watt_hours_imported_callback)(const void*);
  uint16_t (*tot_wh_sf_callback)(const void*);
  uint32_t (*total_va_hours_exported_callback)(const void*);
  uint32_t (*total_va_hours_imported_callback)(const void*);
  uint16_t (*tot_v_ah_sf_callback)(const void*);
  uint32_t (*total_var_hours_imported_q1_callback)(const void*);
  uint32_t (*total_v_ar_hours_imported_q2_callback)(const void*);
  uint32_t (*total_v_ar_hours_exported_q3_callback)(const void*);
  uint32_t (*total_v_ar_hours_exported_q4_callback)(const void*);
  uint16_t (*tot_v_arh_sf_callback)(const void*);
  uint32_t (*events_callback)(const void*);
  uint32_t (*timestamp_callback)(const void*);
  uint16_t (*milliseconds_callback)(const void*);
  uint16_t (*sequence_callback)(const void*);
  Alg (*algorithm_callback)(const void*);
  uint16_t (*n_callback)(const void*);
} Model220CallbackAdapter;

typedef struct Model220StatefulAdapter {
  int16_t amps;
  uint16_t a_sf;
  int16_t voltage;
  uint16_t v_sf;
  int16_t hz;
  uint16_t hz_sf;
  int16_t watts;
  uint16_t w_sf;
  int16_t va;
  uint16_t va_sf;
  int16_t var;
  uint16_t var_sf;
  int16_t pf;
  uint16_t pf_sf;
  uint32_t total_watt_hours_exported;
  uint32_t total_watt_hours_imported;
  uint16_t tot_wh_sf;
  uint32_t total_va_hours_exported;
  uint32_t total_va_hours_imported;
  uint16_t tot_v_ah_sf;
  uint32_t total_var_hours_imported_q1;
  uint32_t total_v_ar_hours_imported_q2;
  uint32_t total_v_ar_hours_exported_q3;
  uint32_t total_v_ar_hours_exported_q4;
  uint16_t tot_v_arh_sf;
  uint32_t events;
  uint32_t timestamp;
  uint16_t milliseconds;
  uint16_t sequence;
  Alg algorithm;
  uint16_t n;
} Model220StatefulAdapter;

typedef struct Model305CallbackAdapter {
  void *context;
  const char *(*tm_callback)(const void*);
  const char *(*date_callback)(const void*);
  const char *(*location_callback)(const void*);
  int32_t (*lat_callback)(const void*);
  int32_t (*long_callback)(const void*);
  int32_t (*altitude_callback)(const void*);
} Model305CallbackAdapter;

typedef struct Model305StatefulAdapter {
  char tm[12];
  char date[8];
  char location[40];
  int32_t lat;
  int32_t long_;
  int32_t altitude;
} Model305StatefulAdapter;

typedef struct Model306CallbackAdapter {
  void *context;
  uint16_t (*ghi_callback)(const void*);
  uint16_t (*amps_callback)(const void*);
  uint16_t (*voltage_callback)(const void*);
  uint16_t (*temperature_callback)(const void*);
} Model306CallbackAdapter;

typedef struct Model306StatefulAdapter {
  uint16_t ghi;
  uint16_t amps;
  uint16_t voltage;
  uint16_t temperature;
} Model306StatefulAdapter;

typedef struct Model307CallbackAdapter {
  void *context;
  int16_t (*ambient_temperature_callback)(const void*);
  int16_t (*relative_humidity_callback)(const void*);
  int16_t (*barometric_pressure_callback)(const void*);
  int16_t (*wind_speed_callback)(const void*);
  int16_t (*wind_direction_callback)(const void*);
  int16_t (*rainfall_callback)(const void*);
  int16_t (*snow_depth_callback)(const void*);
  int16_t (*precipitation_type_callback)(const void*);
  int16_t (*electric_field_callback)(const void*);
  int16_t (*surface_wetness_callback)(const void*);
  int16_t (*soil_wetness_callback)(const void*);
} Model307CallbackAdapter;

typedef struct Model307StatefulAdapter {
  int16_t ambient_temperature;
  int16_t relative_humidity;
  int16_t barometric_pressure;
  int16_t wind_speed;
  int16_t wind_direction;
  int16_t rainfall;
  int16_t snow_depth;
  int16_t precipitation_type;
  int16_t electric_field;
  int16_t surface_wetness;
  int16_t soil_wetness;
} Model307StatefulAdapter;

typedef struct Model308CallbackAdapter {
  void *context;
  uint16_t (*ghi_callback)(const void*);
  int16_t (*temp_callback)(const void*);
  int16_t (*ambient_temperature_callback)(const void*);
  uint16_t (*wind_speed_callback)(const void*);
} Model308CallbackAdapter;

typedef struct Model308StatefulAdapter {
  uint16_t ghi;
  int16_t temp;
  int16_t ambient_temperature;
  uint16_t wind_speed;
} Model308StatefulAdapter;

typedef struct Model401CallbackAdapter {
  void *context;
  uint16_t (*dca_sf_callback)(const void*);
  uint16_t (*dc_ahr_sf_callback)(const void*);
  uint16_t (*dcv_sf_callback)(const void*);
  uint16_t (*rating_callback)(const void*);
  uint16_t (*n_callback)(const void*);
  uint32_t (*event_callback)(const void*);
  uint32_t (*vendor_event_callback)(const void*);
  int16_t (*amps_callback)(const void*);
  uint32_t (*amp_hours_callback)(const void*);
  uint16_t (*voltage_callback)(const void*);
  int16_t (*temp_callback)(const void*);
} Model401CallbackAdapter;

typedef struct Model401StatefulAdapter {
  uint16_t dca_sf;
  uint16_t dc_ahr_sf;
  uint16_t dcv_sf;
  uint16_t rating;
  uint16_t n;
  uint32_t event;
  uint32_t vendor_event;
  int16_t amps;
  uint32_t amp_hours;
  uint16_t voltage;
  int16_t temp;
} Model401StatefulAdapter;

typedef struct Model402CallbackAdapter {
  void *context;
  uint16_t (*dca_sf_callback)(const void*);
  uint16_t (*dc_ahr_sf_callback)(const void*);
  uint16_t (*dcv_sf_callback)(const void*);
  uint16_t (*dcw_sf_callback)(const void*);
  uint16_t (*dc_wh_sf_callback)(const void*);
  uint16_t (*rating_callback)(const void*);
  uint16_t (*n_callback)(const void*);
  uint32_t (*event_callback)(const void*);
  uint32_t (*vendor_event_callback)(const void*);
  int16_t (*amps_callback)(const void*);
  uint32_t (*amp_hours_callback)(const void*);
  uint16_t (*voltage_callback)(const void*);
  int16_t (*temp_callback)(const void*);
  int16_t (*watts_callback)(const void*);
  uint16_t (*pr_callback)(const void*);
  uint32_t (*watt_hours_callback)(const void*);
} Model402CallbackAdapter;

typedef struct Model402StatefulAdapter {
  uint16_t dca_sf;
  uint16_t dc_ahr_sf;
  uint16_t dcv_sf;
  uint16_t dcw_sf;
  uint16_t dc_wh_sf;
  uint16_t rating;
  uint16_t n;
  uint32_t event;
  uint32_t vendor_event;
  int16_t amps;
  uint32_t amp_hours;
  uint16_t voltage;
  int16_t temp;
  int16_t watts;
  uint16_t pr;
  uint32_t watt_hours;
} Model402StatefulAdapter;

typedef struct Model403CallbackAdapter {
  void *context;
  uint16_t (*dca_sf_callback)(const void*);
  uint16_t (*dc_ahr_sf_callback)(const void*);
  uint16_t (*dcv_sf_callback)(const void*);
  uint16_t (*rating_callback)(const void*);
  uint16_t (*n_callback)(const void*);
  uint32_t (*event_callback)(const void*);
  uint32_t (*vendor_event_callback)(const void*);
  int16_t (*amps_callback)(const void*);
  uint32_t (*amp_hours_callback)(const void*);
  int16_t (*voltage_callback)(const void*);
  int16_t (*temp_callback)(const void*);
  uint16_t (*in_dca_sf_callback)(const void*);
  uint16_t (*in_dc_ahr_sf_callback)(const void*);
} Model403CallbackAdapter;

typedef struct Model403StatefulAdapter {
  uint16_t dca_sf;
  uint16_t dc_ahr_sf;
  uint16_t dcv_sf;
  uint16_t rating;
  uint16_t n;
  uint32_t event;
  uint32_t vendor_event;
  int16_t amps;
  uint32_t amp_hours;
  int16_t voltage;
  int16_t temp;
  uint16_t in_dca_sf;
  uint16_t in_dc_ahr_sf;
} Model403StatefulAdapter;

typedef struct Model404CallbackAdapter {
  void *context;
  uint16_t (*dca_sf_callback)(const void*);
  uint16_t (*dc_ahr_sf_callback)(const void*);
  uint16_t (*dcv_sf_callback)(const void*);
  uint16_t (*dcw_sf_callback)(const void*);
  uint16_t (*dc_wh_sf_callback)(const void*);
  uint16_t (*rating_callback)(const void*);
  uint16_t (*n_callback)(const void*);
  uint32_t (*event_callback)(const void*);
  uint32_t (*vendor_event_callback)(const void*);
  int16_t (*amps_callback)(const void*);
  uint32_t (*amp_hours_callback)(const void*);
  int16_t (*voltage_callback)(const void*);
  int16_t (*temp_callback)(const void*);
  int16_t (*watts_callback)(const void*);
  int16_t (*pr_callback)(const void*);
  uint32_t (*watt_hours_callback)(const void*);
  uint16_t (*in_dca_sf_callback)(const void*);
  uint16_t (*in_dc_ahr_sf_callback)(const void*);
  uint16_t (*in_dcv_sf_callback)(const void*);
  uint16_t (*in_dcw_sf_callback)(const void*);
  uint16_t (*in_dc_wh_sf_callback)(const void*);
} Model404CallbackAdapter;

typedef struct Model404StatefulAdapter {
  uint16_t dca_sf;
  uint16_t dc_ahr_sf;
  uint16_t dcv_sf;
  uint16_t dcw_sf;
  uint16_t dc_wh_sf;
  uint16_t rating;
  uint16_t n;
  uint32_t event;
  uint32_t vendor_event;
  int16_t amps;
  uint32_t amp_hours;
  int16_t voltage;
  int16_t temp;
  int16_t watts;
  int16_t pr;
  uint32_t watt_hours;
  uint16_t in_dca_sf;
  uint16_t in_dc_ahr_sf;
  uint16_t in_dcv_sf;
  uint16_t in_dcw_sf;
  uint16_t in_dc_wh_sf;
} Model404StatefulAdapter;

typedef struct Model501CallbackAdapter {
  void *context;
  Stat (*status_callback)(const void*);
  uint16_t (*vendor_status_callback)(const void*);
  uint32_t (*events_callback)(const void*);
  uint32_t (*vendor_module_event_flags_callback)(const void*);
  uint16_t (*control_callback)(const void*);
  void (*set_control_callback)(uint16_t, void*);
  uint32_t (*vendor_control_callback)(const void*);
  void (*set_vendor_control_callback)(uint32_t, void*);
  int32_t (*control_value_callback)(const void*);
  void (*set_control_value_callback)(int32_t, void*);
  uint32_t (*timestamp_callback)(const void*);
  float (*output_current_callback)(const void*);
  float (*output_voltage_callback)(const void*);
  float (*output_energy_callback)(const void*);
  float (*output_power_callback)(const void*);
  float (*temp_callback)(const void*);
  float (*input_current_callback)(const void*);
  float (*input_voltage_callback)(const void*);
  float (*input_energy_callback)(const void*);
  float (*input_power_callback)(const void*);
} Model501CallbackAdapter;

typedef struct Model501StatefulAdapter {
  Stat status;
  uint16_t vendor_status;
  uint32_t events;
  uint32_t vendor_module_event_flags;
  uint16_t control;
  uint32_t vendor_control;
  int32_t control_value;
  uint32_t timestamp;
  float output_current;
  float output_voltage;
  float output_energy;
  float output_power;
  float temp;
  float input_current;
  float input_voltage;
  float input_energy;
  float input_power;
} Model501StatefulAdapter;

typedef struct Model502CallbackAdapter {
  void *context;
  uint16_t (*a_sf_callback)(const void*);
  uint16_t (*v_sf_callback)(const void*);
  uint16_t (*w_sf_callback)(const void*);
  uint16_t (*wh_sf_callback)(const void*);
  Stat (*status_callback)(const void*);
  uint16_t (*vendor_status_callback)(const void*);
  uint32_t (*events_callback)(const void*);
  uint32_t (*vendor_module_event_flags_callback)(const void*);
  uint16_t (*control_callback)(const void*);
  void (*set_control_callback)(uint16_t, void*);
  uint32_t (*vendor_control_callback)(const void*);
  void (*set_vendor_control_callback)(uint32_t, void*);
  int32_t (*control_value_callback)(const void*);
  void (*set_control_value_callback)(int32_t, void*);
  uint32_t (*timestamp_callback)(const void*);
  int16_t (*output_current_callback)(const void*);
  int16_t (*output_voltage_callback)(const void*);
  uint32_t (*output_energy_callback)(const void*);
  int16_t (*output_power_callback)(const void*);
  int16_t (*temp_callback)(const void*);
  int16_t (*input_current_callback)(const void*);
  int16_t (*input_voltage_callback)(const void*);
  uint32_t (*input_energy_callback)(const void*);
  int16_t (*input_power_callback)(const void*);
} Model502CallbackAdapter;

typedef struct Model502StatefulAdapter {
  uint16_t a_sf;
  uint16_t v_sf;
  uint16_t w_sf;
  uint16_t wh_sf;
  Stat status;
  uint16_t vendor_status;
  uint32_t events;
  uint32_t vendor_module_event_flags;
  uint16_t control;
  uint32_t vendor_control;
  int32_t control_value;
  uint32_t timestamp;
  int16_t output_current;
  int16_t output_voltage;
  uint32_t output_energy;
  int16_t output_power;
  int16_t temp;
  int16_t input_current;
  int16_t input_voltage;
  uint32_t input_energy;
  int16_t input_power;
} Model502StatefulAdapter;

typedef struct Model701CallbackAdapter {
  void *context;
  AcType (*ac_wiring_type_callback)(const void*);
  St (*operating_state_callback)(const void*);
  InvSt (*inverter_state_callback)(const void*);
  ConnSt (*grid_connection_state_callback)(const void*);
  uint32_t (*alarm_bitfield_callback)(const void*);
  uint32_t (*der_operational_characteristics_callback)(const void*);
  int16_t (*active_power_callback)(const void*);
  int16_t (*apparent_power_callback)(const void*);
  int16_t (*reactive_power_callback)(const void*);
  int16_t (*power_factor_callback)(const void*);
  int16_t (*total_ac_current_callback)(const void*);
  uint16_t (*voltage_ll_callback)(const void*);
  uint16_t (*voltage_ln_callback)(const void*);
  uint32_t (*frequency_callback)(const void*);
  uint64_t (*total_energy_injected_callback)(const void*);
  uint64_t (*total_energy_absorbed_callback)(const void*);
  uint64_t (*total_reactive_energy_inj_callback)(const void*);
  uint64_t (*total_reactive_energy_abs_callback)(const void*);
  int16_t (*ambient_temperature_callback)(const void*);
  int16_t (*cabinet_temperature_callback)(const void*);
  int16_t (*heat_sink_temperature_callback)(const void*);
  int16_t (*transformer_temperature_callback)(const void*);
  int16_t (*igbt_mosfet_temperature_callback)(const void*);
  int16_t (*other_temperature_callback)(const void*);
  int16_t (*watts_l1_callback)(const void*);
  int16_t (*va_l1_callback)(const void*);
  int16_t (*var_l1_callback)(const void*);
  int16_t (*pf_l1_callback)(const void*);
  int16_t (*amps_l1_callback)(const void*);
  uint16_t (*phase_voltage_l1_l2_callback)(const void*);
  uint16_t (*phase_voltage_l1_n_callback)(const void*);
  uint64_t (*total_watt_hours_inj_l1_callback)(const void*);
  uint64_t (*total_watt_hours_abs_l1_callback)(const void*);
  uint64_t (*total_var_hours_inj_l1_callback)(const void*);
  uint64_t (*total_var_hours_abs_l1_callback)(const void*);
  int16_t (*watts_l2_callback)(const void*);
  int16_t (*va_l2_callback)(const void*);
  int16_t (*var_l2_callback)(const void*);
  int16_t (*pf_l2_callback)(const void*);
  int16_t (*amps_l2_callback)(const void*);
  uint16_t (*phase_voltage_l2_l3_callback)(const void*);
  uint16_t (*phase_voltage_l2_n_callback)(const void*);
  uint64_t (*total_watt_hours_inj_l2_callback)(const void*);
  uint64_t (*total_watt_hours_abs_l2_callback)(const void*);
  uint64_t (*total_var_hours_inj_l2_callback)(const void*);
  uint64_t (*total_var_hours_abs_l2_callback)(const void*);
  int16_t (*watts_l3_callback)(const void*);
  int16_t (*va_l3_callback)(const void*);
  int16_t (*var_l3_callback)(const void*);
  int16_t (*pf_l3_callback)(const void*);
  int16_t (*amps_l3_callback)(const void*);
  uint16_t (*phase_voltage_l3_l1_callback)(const void*);
  uint16_t (*phase_voltage_l3_n_callback)(const void*);
  uint64_t (*total_watt_hours_inj_l3_callback)(const void*);
  uint64_t (*total_watt_hours_abs_l3_callback)(const void*);
  uint64_t (*total_var_hours_inj_l3_callback)(const void*);
  uint64_t (*total_var_hours_abs_l3_callback)(const void*);
  uint16_t (*throttling_in_pct_callback)(const void*);
  uint32_t (*throttle_source_information_callback)(const void*);
  uint16_t (*current_scale_factor_callback)(const void*);
  uint16_t (*voltage_scale_factor_callback)(const void*);
  uint16_t (*frequency_scale_factor_callback)(const void*);
  uint16_t (*active_power_scale_factor_callback)(const void*);
  uint16_t (*power_factor_scale_factor_callback)(const void*);
  uint16_t (*apparent_power_scale_factor_callback)(const void*);
  uint16_t (*reactive_power_scale_factor_callback)(const void*);
  uint16_t (*active_energy_scale_factor_callback)(const void*);
  uint16_t (*reactive_energy_scale_factor_callback)(const void*);
  uint16_t (*temperature_scale_factor_callback)(const void*);
  const char *(*manufacturer_alarm_info_callback)(const void*);
} Model701CallbackAdapter;

typedef struct Model701StatefulAdapter {
  AcType ac_wiring_type;
  St operating_state;
  InvSt inverter_state;
  ConnSt grid_connection_state;
  uint32_t alarm_bitfield;
  uint32_t der_operational_characteristics;
  int16_t active_power;
  int16_t apparent_power;
  int16_t reactive_power;
  int16_t power_factor;
  int16_t total_ac_current;
  uint16_t voltage_ll;
  uint16_t voltage_ln;
  uint32_t frequency;
  uint64_t total_energy_injected;
  uint64_t total_energy_absorbed;
  uint64_t total_reactive_energy_inj;
  uint64_t total_reactive_energy_abs;
  int16_t ambient_temperature;
  int16_t cabinet_temperature;
  int16_t heat_sink_temperature;
  int16_t transformer_temperature;
  int16_t igbt_mosfet_temperature;
  int16_t other_temperature;
  int16_t watts_l1;
  int16_t va_l1;
  int16_t var_l1;
  int16_t pf_l1;
  int16_t amps_l1;
  uint16_t phase_voltage_l1_l2;
  uint16_t phase_voltage_l1_n;
  uint64_t total_watt_hours_inj_l1;
  uint64_t total_watt_hours_abs_l1;
  uint64_t total_var_hours_inj_l1;
  uint64_t total_var_hours_abs_l1;
  int16_t watts_l2;
  int16_t va_l2;
  int16_t var_l2;
  int16_t pf_l2;
  int16_t amps_l2;
  uint16_t phase_voltage_l2_l3;
  uint16_t phase_voltage_l2_n;
  uint64_t total_watt_hours_inj_l2;
  uint64_t total_watt_hours_abs_l2;
  uint64_t total_var_hours_inj_l2;
  uint64_t total_var_hours_abs_l2;
  int16_t watts_l3;
  int16_t va_l3;
  int16_t var_l3;
  int16_t pf_l3;
  int16_t amps_l3;
  uint16_t phase_voltage_l3_l1;
  uint16_t phase_voltage_l3_n;
  uint64_t total_watt_hours_inj_l3;
  uint64_t total_watt_hours_abs_l3;
  uint64_t total_var_hours_inj_l3;
  uint64_t total_var_hours_abs_l3;
  uint16_t throttling_in_pct;
  uint32_t throttle_source_information;
  uint16_t current_scale_factor;
  uint16_t voltage_scale_factor;
  uint16_t frequency_scale_factor;
  uint16_t active_power_scale_factor;
  uint16_t power_factor_scale_factor;
  uint16_t apparent_power_scale_factor;
  uint16_t reactive_power_scale_factor;
  uint16_t active_energy_scale_factor;
  uint16_t reactive_energy_scale_factor;
  uint16_t temperature_scale_factor;
  char manufacturer_alarm_info[64];
} Model701StatefulAdapter;

typedef struct Model703CallbackAdapter {
  void *context;
  Es (*permit_enter_service_callback)(const void*);
  void (*set_permit_enter_service_callback)(Es, void*);
  uint16_t (*enter_service_voltage_high_callback)(const void*);
  void (*set_enter_service_voltage_high_callback)(uint16_t, void*);
  uint16_t (*enter_service_voltage_low_callback)(const void*);
  void (*set_enter_service_voltage_low_callback)(uint16_t, void*);
  uint32_t (*enter_service_frequency_high_callback)(const void*);
  void (*set_enter_service_frequency_high_callback)(uint32_t, void*);
  uint32_t (*enter_service_frequency_low_callback)(const void*);
  void (*set_enter_service_frequency_low_callback)(uint32_t, void*);
  uint32_t (*enter_service_delay_time_callback)(const void*);
  void (*set_enter_service_delay_time_callback)(uint32_t, void*);
  uint32_t (*enter_service_random_delay_callback)(const void*);
  void (*set_enter_service_random_delay_callback)(uint32_t, void*);
  uint32_t (*enter_service_ramp_time_callback)(const void*);
  void (*set_enter_service_ramp_time_callback)(uint32_t, void*);
  uint32_t (*enter_service_delay_remaining_callback)(const void*);
  uint16_t (*voltage_scale_factor_callback)(const void*);
  uint16_t (*frequency_scale_factor_callback)(const void*);
} Model703CallbackAdapter;

typedef struct Model703StatefulAdapter {
  Es permit_enter_service;
  uint16_t enter_service_voltage_high;
  uint16_t enter_service_voltage_low;
  uint32_t enter_service_frequency_high;
  uint32_t enter_service_frequency_low;
  uint32_t enter_service_delay_time;
  uint32_t enter_service_random_delay;
  uint32_t enter_service_ramp_time;
  uint32_t enter_service_delay_remaining;
  uint16_t voltage_scale_factor;
  uint16_t frequency_scale_factor;
} Model703StatefulAdapter;

typedef struct Model704CallbackAdapter {
  void *context;
  PfwInjEna (*power_factor_enable_w_inj_enable_callback)(const void*);
  void (*set_power_factor_enable_w_inj_enable_callback)(PfwInjEna, void*);
  PfwInjEnaRvrt (*power_factor_reversion_enable_w_inj_callback)(const void*);
  void (*set_power_factor_reversion_enable_w_inj_callback)(PfwInjEnaRvrt, void*);
  uint32_t (*pf_reversion_time_w_inj_callback)(const void*);
  void (*set_pf_reversion_time_w_inj_callback)(uint32_t, void*);
  uint32_t (*pf_reversion_time_rem_w_inj_callback)(const void*);
  PfwAbsEna (*power_factor_enable_w_abs_enable_callback)(const void*);
  void (*set_power_factor_enable_w_abs_enable_callback)(PfwAbsEna, void*);
  PfwAbsEnaRvrt (*power_factor_reversion_enable_w_abs_callback)(const void*);
  void (*set_power_factor_reversion_enable_w_abs_callback)(PfwAbsEnaRvrt, void*);
  uint32_t (*pf_reversion_time_w_abs_callback)(const void*);
  void (*set_pf_reversion_time_w_abs_callback)(uint32_t, void*);
  uint32_t (*pf_reversion_time_rem_w_abs_callback)(const void*);
  WMaxLimPctEna (*limit_max_power_pct_enable_callback)(const void*);
  void (*set_limit_max_power_pct_enable_callback)(WMaxLimPctEna, void*);
  uint16_t (*limit_max_power_pct_setpoint_callback)(const void*);
  void (*set_limit_max_power_pct_setpoint_callback)(uint16_t, void*);
  uint16_t (*reversion_limit_max_power_pct_callback)(const void*);
  void (*set_reversion_limit_max_power_pct_callback)(uint16_t, void*);
  WMaxLimPctEnaRvrt (*reversion_limit_max_power_pct_enable_callback)(const void*);
  void (*set_reversion_limit_max_power_pct_enable_callback)(WMaxLimPctEnaRvrt, void*);
  uint32_t (*limit_max_power_pct_reversion_time_callback)(const void*);
  void (*set_limit_max_power_pct_reversion_time_callback)(uint32_t, void*);
  uint32_t (*limit_max_power_pct_rev_time_rem_callback)(const void*);
  WSetEna (*active_power_enable_callback)(const void*);
  void (*set_active_power_enable_callback)(WSetEna, void*);
  WSetMod (*active_power_mode_callback)(const void*);
  void (*set_active_power_mode_callback)(WSetMod, void*);
  int32_t (*active_power_setpoint_w_callback)(const void*);
  void (*set_active_power_setpoint_w_callback)(int32_t, void*);
  int32_t (*reversion_active_power_w_callback)(const void*);
  void (*set_reversion_active_power_w_callback)(int32_t, void*);
  int16_t (*active_power_setpoint_pct_callback)(const void*);
  void (*set_active_power_setpoint_pct_callback)(int16_t, void*);
  int16_t (*reversion_active_power_pct_callback)(const void*);
  void (*set_reversion_active_power_pct_callback)(int16_t, void*);
  WSetEnaRvrt (*reversion_active_power_enable_callback)(const void*);
  void (*set_reversion_active_power_enable_callback)(WSetEnaRvrt, void*);
  uint32_t (*active_power_reversion_time_callback)(const void*);
  void (*set_active_power_reversion_time_callback)(uint32_t, void*);
  uint32_t (*active_power_rev_time_rem_callback)(const void*);
  VarSetEna (*reactive_power_enable_callback)(const void*);
  void (*set_reactive_power_enable_callback)(VarSetEna, void*);
  VarSetMod (*reactive_power_mode_callback)(const void*);
  void (*set_reactive_power_mode_callback)(VarSetMod, void*);
  VarSetPri (*reactive_power_priority_callback)(const void*);
  void (*set_reactive_power_priority_callback)(VarSetPri, void*);
  int32_t (*reactive_power_setpoint_vars_callback)(const void*);
  void (*set_reactive_power_setpoint_vars_callback)(int32_t, void*);
  int32_t (*reversion_reactive_power_vars_callback)(const void*);
  void (*set_reversion_reactive_power_vars_callback)(int32_t, void*);
  int16_t (*reactive_power_setpoint_pct_callback)(const void*);
  void (*set_reactive_power_setpoint_pct_callback)(int16_t, void*);
  int16_t (*reversion_reactive_power_pct_callback)(const void*);
  void (*set_reversion_reactive_power_pct_callback)(int16_t, void*);
  VarSetEnaRvrt (*reversion_reactive_power_enable_callback)(const void*);
  void (*set_reversion_reactive_power_enable_callback)(VarSetEnaRvrt, void*);
  uint32_t (*reactive_power_reversion_time_callback)(const void*);
  void (*set_reactive_power_reversion_time_callback)(uint32_t, void*);
  uint32_t (*reactive_power_rev_time_rem_callback)(const void*);
  uint16_t (*normal_ramp_rate_callback)(const void*);
  void (*set_normal_ramp_rate_callback)(uint16_t, void*);
  WRmpRef (*normal_ramp_rate_reference_callback)(const void*);
  void (*set_normal_ramp_rate_reference_callback)(WRmpRef, void*);
  uint16_t (*reactive_power_ramp_rate_callback)(const void*);
  void (*set_reactive_power_ramp_rate_callback)(uint16_t, void*);
  AntiIslEna (*anti_islanding_enable_callback)(const void*);
  void (*set_anti_islanding_enable_callback)(AntiIslEna, void*);
  uint16_t (*power_factor_scale_factor_callback)(const void*);
  uint16_t (*limit_max_power_scale_factor_callback)(const void*);
  uint16_t (*active_power_scale_factor_callback)(const void*);
  uint16_t (*active_power_pct_scale_factor_callback)(const void*);
  uint16_t (*reactive_power_scale_factor_callback)(const void*);
  uint16_t (*reactive_power_pct_scale_factor_callback)(const void*);
} Model704CallbackAdapter;

typedef struct Model704StatefulAdapter {
  PfwInjEna power_factor_enable_w_inj_enable;
  PfwInjEnaRvrt power_factor_reversion_enable_w_inj;
  uint32_t pf_reversion_time_w_inj;
  uint32_t pf_reversion_time_rem_w_inj;
  PfwAbsEna power_factor_enable_w_abs_enable;
  PfwAbsEnaRvrt power_factor_reversion_enable_w_abs;
  uint32_t pf_reversion_time_w_abs;
  uint32_t pf_reversion_time_rem_w_abs;
  WMaxLimPctEna limit_max_power_pct_enable;
  uint16_t limit_max_power_pct_setpoint;
  uint16_t reversion_limit_max_power_pct;
  WMaxLimPctEnaRvrt reversion_limit_max_power_pct_enable;
  uint32_t limit_max_power_pct_reversion_time;
  uint32_t limit_max_power_pct_rev_time_rem;
  WSetEna active_power_enable;
  WSetMod active_power_mode;
  int32_t active_power_setpoint_w;
  int32_t reversion_active_power_w;
  int16_t active_power_setpoint_pct;
  int16_t reversion_active_power_pct;
  WSetEnaRvrt reversion_active_power_enable;
  uint32_t active_power_reversion_time;
  uint32_t active_power_rev_time_rem;
  VarSetEna reactive_power_enable;
  VarSetMod reactive_power_mode;
  VarSetPri reactive_power_priority;
  int32_t reactive_power_setpoint_vars;
  int32_t reversion_reactive_power_vars;
  int16_t reactive_power_setpoint_pct;
  int16_t reversion_reactive_power_pct;
  VarSetEnaRvrt reversion_reactive_power_enable;
  uint32_t reactive_power_reversion_time;
  uint32_t reactive_power_rev_time_rem;
  uint16_t normal_ramp_rate;
  WRmpRef normal_ramp_rate_reference;
  uint16_t reactive_power_ramp_rate;
  AntiIslEna anti_islanding_enable;
  uint16_t power_factor_scale_factor;
  uint16_t limit_max_power_scale_factor;
  uint16_t active_power_scale_factor;
  uint16_t active_power_pct_scale_factor;
  uint16_t reactive_power_scale_factor;
  uint16_t reactive_power_pct_scale_factor;
} Model704StatefulAdapter;

typedef struct Model705CallbackAdapter {
  void *context;
  Ena (*der_volt_var_module_enable_callback)(const void*);
  void (*set_der_volt_var_module_enable_callback)(Ena, void*);
  uint16_t (*adopt_curve_request_callback)(const void*);
  void (*set_adopt_curve_request_callback)(uint16_t, void*);
  AdptCrvRslt (*adopt_curve_result_callback)(const void*);
  uint16_t (*number_of_points_callback)(const void*);
  uint16_t (*stored_curve_count_callback)(const void*);
  uint32_t (*reversion_timeout_callback)(const void*);
  void (*set_reversion_timeout_callback)(uint32_t, void*);
  uint32_t (*reversion_time_remaining_callback)(const void*);
  uint16_t (*reversion_curve_callback)(const void*);
  void (*set_reversion_curve_callback)(uint16_t, void*);
  uint16_t (*voltage_scale_factor_callback)(const void*);
  uint16_t (*var_scale_factor_callback)(const void*);
  uint16_t (*open_loop_scale_factor_callback)(const void*);
} Model705CallbackAdapter;

typedef struct Model705StatefulAdapter {
  Ena der_volt_var_module_enable;
  uint16_t adopt_curve_request;
  AdptCrvRslt adopt_curve_result;
  uint16_t number_of_points;
  uint16_t stored_curve_count;
  uint32_t reversion_timeout;
  uint32_t reversion_time_remaining;
  uint16_t reversion_curve;
  uint16_t voltage_scale_factor;
  uint16_t var_scale_factor;
  uint16_t open_loop_scale_factor;
} Model705StatefulAdapter;

typedef struct Model706CallbackAdapter {
  void *context;
  Ena (*der_volt_watt_module_enable_callback)(const void*);
  void (*set_der_volt_watt_module_enable_callback)(Ena, void*);
  uint16_t (*adopt_curve_request_callback)(const void*);
  void (*set_adopt_curve_request_callback)(uint16_t, void*);
  AdptCrvRslt (*adopt_curve_result_callback)(const void*);
  uint16_t (*number_of_points_callback)(const void*);
  uint16_t (*stored_curve_count_callback)(const void*);
  uint32_t (*reversion_timeout_callback)(const void*);
  void (*set_reversion_timeout_callback)(uint32_t, void*);
  uint32_t (*reversion_time_remaining_callback)(const void*);
  uint16_t (*reversion_curve_callback)(const void*);
  void (*set_reversion_curve_callback)(uint16_t, void*);
  uint16_t (*voltage_scale_factor_callback)(const void*);
  uint16_t (*watt_scale_factor_callback)(const void*);
  uint16_t (*open_loop_scale_factor_callback)(const void*);
} Model706CallbackAdapter;

typedef struct Model706StatefulAdapter {
  Ena der_volt_watt_module_enable;
  uint16_t adopt_curve_request;
  AdptCrvRslt adopt_curve_result;
  uint16_t number_of_points;
  uint16_t stored_curve_count;
  uint32_t reversion_timeout;
  uint32_t reversion_time_remaining;
  uint16_t reversion_curve;
  uint16_t voltage_scale_factor;
  uint16_t watt_scale_factor;
  uint16_t open_loop_scale_factor;
} Model706StatefulAdapter;

typedef struct Model707CallbackAdapter {
  void *context;
  Ena (*der_trip_lv_module_enable_callback)(const void*);
  void (*set_der_trip_lv_module_enable_callback)(Ena, void*);
  uint16_t (*adopt_curve_request_callback)(const void*);
  void (*set_adopt_curve_request_callback)(uint16_t, void*);
  AdptCrvRslt (*adopt_curve_result_callback)(const void*);
  uint16_t (*number_of_points_callback)(const void*);
  uint16_t (*stored_curve_count_callback)(const void*);
  uint16_t (*voltage_scale_factor_callback)(const void*);
  uint16_t (*time_point_scale_factor_callback)(const void*);
} Model707CallbackAdapter;

typedef struct Model707StatefulAdapter {
  Ena der_trip_lv_module_enable;
  uint16_t adopt_curve_request;
  AdptCrvRslt adopt_curve_result;
  uint16_t number_of_points;
  uint16_t stored_curve_count;
  uint16_t voltage_scale_factor;
  uint16_t time_point_scale_factor;
} Model707StatefulAdapter;

typedef struct Model708CallbackAdapter {
  void *context;
  Ena (*der_trip_hv_module_enable_callback)(const void*);
  void (*set_der_trip_hv_module_enable_callback)(Ena, void*);
  uint16_t (*adopt_curve_request_callback)(const void*);
  void (*set_adopt_curve_request_callback)(uint16_t, void*);
  AdptCrvRslt (*adopt_curve_result_callback)(const void*);
  uint16_t (*number_of_points_callback)(const void*);
  uint16_t (*stored_curve_count_callback)(const void*);
  uint16_t (*voltage_scale_factor_callback)(const void*);
  uint16_t (*time_point_scale_factor_callback)(const void*);
} Model708CallbackAdapter;

typedef struct Model708StatefulAdapter {
  Ena der_trip_hv_module_enable;
  uint16_t adopt_curve_request;
  AdptCrvRslt adopt_curve_result;
  uint16_t number_of_points;
  uint16_t stored_curve_count;
  uint16_t voltage_scale_factor;
  uint16_t time_point_scale_factor;
} Model708StatefulAdapter;

typedef struct Model709CallbackAdapter {
  void *context;
  Ena (*der_trip_lf_module_enable_callback)(const void*);
  void (*set_der_trip_lf_module_enable_callback)(Ena, void*);
  uint16_t (*adopt_curve_request_callback)(const void*);
  void (*set_adopt_curve_request_callback)(uint16_t, void*);
  AdptCrvRslt (*adopt_curve_result_callback)(const void*);
  uint16_t (*number_of_points_callback)(const void*);
  uint16_t (*stored_curve_count_callback)(const void*);
  uint16_t (*frequency_scale_factor_callback)(const void*);
  uint16_t (*time_point_scale_factor_callback)(const void*);
} Model709CallbackAdapter;

typedef struct Model709StatefulAdapter {
  Ena der_trip_lf_module_enable;
  uint16_t adopt_curve_request;
  AdptCrvRslt adopt_curve_result;
  uint16_t number_of_points;
  uint16_t stored_curve_count;
  uint16_t frequency_scale_factor;
  uint16_t time_point_scale_factor;
} Model709StatefulAdapter;

typedef struct Model710CallbackAdapter {
  void *context;
  Ena (*der_trip_hf_module_enable_callback)(const void*);
  void (*set_der_trip_hf_module_enable_callback)(Ena, void*);
  uint16_t (*adopt_curve_request_callback)(const void*);
  void (*set_adopt_curve_request_callback)(uint16_t, void*);
  AdptCrvRslt (*adopt_curve_result_callback)(const void*);
  uint16_t (*number_of_points_callback)(const void*);
  uint16_t (*stored_curve_count_callback)(const void*);
  uint16_t (*frequency_scale_factor_callback)(const void*);
  uint16_t (*time_point_scale_factor_callback)(const void*);
} Model710CallbackAdapter;

typedef struct Model710StatefulAdapter {
  Ena der_trip_hf_module_enable;
  uint16_t adopt_curve_request;
  AdptCrvRslt adopt_curve_result;
  uint16_t number_of_points;
  uint16_t stored_curve_count;
  uint16_t frequency_scale_factor;
  uint16_t time_point_scale_factor;
} Model710StatefulAdapter;

typedef struct Model711CallbackAdapter {
  void *context;
  Ena (*der_frequency_droop_module_enable_callback)(const void*);
  void (*set_der_frequency_droop_module_enable_callback)(Ena, void*);
  uint16_t (*active_control_request_callback)(const void*);
  void (*set_active_control_request_callback)(uint16_t, void*);
  AdptCtlRslt (*set_active_control_result_callback)(const void*);
  uint16_t (*stored_control_count_callback)(const void*);
  uint32_t (*reversion_timeout_callback)(const void*);
  void (*set_reversion_timeout_callback)(uint32_t, void*);
  uint32_t (*reversion_time_left_callback)(const void*);
  uint16_t (*reversion_control_callback)(const void*);
  void (*set_reversion_control_callback)(uint16_t, void*);
  uint16_t (*deadband_scale_factor_callback)(const void*);
  uint16_t (*frequency_change_scale_factor_callback)(const void*);
  uint16_t (*open_loop_scale_factor_callback)(const void*);
} Model711CallbackAdapter;

typedef struct Model711StatefulAdapter {
  Ena der_frequency_droop_module_enable;
  uint16_t active_control_request;
  AdptCtlRslt set_active_control_result;
  uint16_t stored_control_count;
  uint32_t reversion_timeout;
  uint32_t reversion_time_left;
  uint16_t reversion_control;
  uint16_t deadband_scale_factor;
  uint16_t frequency_change_scale_factor;
  uint16_t open_loop_scale_factor;
} Model711StatefulAdapter;

typedef struct Model712CallbackAdapter {
  void *context;
  Ena (*der_watt_var_module_enable_callback)(const void*);
  void (*set_der_watt_var_module_enable_callback)(Ena, void*);
  uint16_t (*active_curve_request_callback)(const void*);
  void (*set_active_curve_request_callback)(uint16_t, void*);
  AdptCrvRslt (*set_active_curve_result_callback)(const void*);
  uint16_t (*number_of_points_callback)(const void*);
  uint16_t (*stored_curve_count_callback)(const void*);
  uint32_t (*reversion_timeout_callback)(const void*);
  void (*set_reversion_timeout_callback)(uint32_t, void*);
  uint32_t (*reversion_time_left_callback)(const void*);
  uint16_t (*reversion_curve_callback)(const void*);
  void (*set_reversion_curve_callback)(uint16_t, void*);
  uint16_t (*active_power_scale_factor_callback)(const void*);
  uint16_t (*var_scale_factor_callback)(const void*);
} Model712CallbackAdapter;

typedef struct Model712StatefulAdapter {
  Ena der_watt_var_module_enable;
  uint16_t active_curve_request;
  AdptCrvRslt set_active_curve_result;
  uint16_t number_of_points;
  uint16_t stored_curve_count;
  uint32_t reversion_timeout;
  uint32_t reversion_time_left;
  uint16_t reversion_curve;
  uint16_t active_power_scale_factor;
  uint16_t var_scale_factor;
} Model712StatefulAdapter;

typedef struct Model713CallbackAdapter {
  void *context;
  uint16_t (*energy_rating_callback)(const void*);
  uint16_t (*energy_available_callback)(const void*);
  uint16_t (*state_of_charge_callback)(const void*);
  uint16_t (*state_of_health_callback)(const void*);
  Sta (*status_callback)(const void*);
  uint16_t (*energy_scale_factor_callback)(const void*);
  uint16_t (*percent_scale_factor_callback)(const void*);
} Model713CallbackAdapter;

typedef struct Model713StatefulAdapter {
  uint16_t energy_rating;
  uint16_t energy_available;
  uint16_t state_of_charge;
  uint16_t state_of_health;
  Sta status;
  uint16_t energy_scale_factor;
  uint16_t percent_scale_factor;
} Model713StatefulAdapter;

typedef struct Model714CallbackAdapter {
  void *context;
  uint32_t (*port_alarms_callback)(const void*);
  uint16_t (*number_of_ports_callback)(const void*);
  int16_t (*dc_current_callback)(const void*);
  int16_t (*dc_power_callback)(const void*);
  uint64_t (*dc_energy_injected_callback)(const void*);
  uint64_t (*dc_energy_absorbed_callback)(const void*);
  uint16_t (*dc_current_scale_factor_callback)(const void*);
  uint16_t (*dc_voltage_scale_factor_callback)(const void*);
  uint16_t (*dc_power_scale_factor_callback)(const void*);
  uint16_t (*dc_energy_scale_factor_callback)(const void*);
  uint16_t (*temperature_scale_factor_callback)(const void*);
} Model714CallbackAdapter;

typedef struct Model714StatefulAdapter {
  uint32_t port_alarms;
  uint16_t number_of_ports;
  int16_t dc_current;
  int16_t dc_power;
  uint64_t dc_energy_injected;
  uint64_t dc_energy_absorbed;
  uint16_t dc_current_scale_factor;
  uint16_t dc_voltage_scale_factor;
  uint16_t dc_power_scale_factor;
  uint16_t dc_energy_scale_factor;
  uint16_t temperature_scale_factor;
} Model714StatefulAdapter;

typedef struct Model715CallbackAdapter {
  void *context;
  LocRemCtl (*control_mode_callback)(const void*);
  uint32_t (*der_heartbeat_callback)(const void*);
  uint32_t (*controller_heartbeat_callback)(const void*);
  void (*set_controller_heartbeat_callback)(uint32_t, void*);
  uint16_t (*alarm_reset_callback)(const void*);
  void (*set_alarm_reset_callback)(uint16_t, void*);
  OpCtl (*operation_callback)(const void*);
  void (*set_operation_callback)(OpCtl, void*);
} Model715CallbackAdapter;

typedef struct Model715StatefulAdapter {
  LocRemCtl control_mode;
  uint32_t der_heartbeat;
  uint32_t controller_heartbeat;
  uint16_t alarm_reset;
  OpCtl operation;
} Model715StatefulAdapter;

typedef struct Model801CallbackAdapter {
  void *context;
  uint16_t (*deprecated_model_callback)(const void*);
} Model801CallbackAdapter;

typedef struct Model801StatefulAdapter {
  uint16_t deprecated_model;
} Model801StatefulAdapter;

typedef struct Model802CallbackAdapter {
  void *context;
  uint16_t (*nameplate_charge_capacity_callback)(const void*);
  uint16_t (*nameplate_energy_capacity_callback)(const void*);
  uint16_t (*nameplate_max_charge_rate_callback)(const void*);
  uint16_t (*nameplate_max_discharge_rate_callback)(const void*);
  uint16_t (*self_discharge_rate_callback)(const void*);
  uint16_t (*nameplate_max_so_c_callback)(const void*);
  uint16_t (*nameplate_min_so_c_callback)(const void*);
  uint16_t (*max_reserve_percent_callback)(const void*);
  void (*set_max_reserve_percent_callback)(uint16_t, void*);
  uint16_t (*min_reserve_percent_callback)(const void*);
  void (*set_min_reserve_percent_callback)(uint16_t, void*);
  uint16_t (*state_of_charge_callback)(const void*);
  uint16_t (*depth_of_discharge_callback)(const void*);
  uint16_t (*state_of_health_callback)(const void*);
  uint32_t (*cycle_count_callback)(const void*);
  ChaSt (*charge_status_callback)(const void*);
  LocRemCtl (*control_mode_callback)(const void*);
  uint16_t (*battery_heartbeat_callback)(const void*);
  uint16_t (*controller_heartbeat_callback)(const void*);
  void (*set_controller_heartbeat_callback)(uint16_t, void*);
  uint16_t (*alarm_reset_callback)(const void*);
  void (*set_alarm_reset_callback)(uint16_t, void*);
  Typ (*battery_type_callback)(const void*);
  State (*state_of_the_battery_bank_callback)(const void*);
  uint16_t (*vendor_battery_bank_state_callback)(const void*);
  uint32_t (*warranty_date_callback)(const void*);
  uint32_t (*battery_event_1_bitfield_callback)(const void*);
  uint32_t (*battery_event_2_bitfield_callback)(const void*);
  uint32_t (*vendor_event_bitfield_1_callback)(const void*);
  uint32_t (*vendor_event_bitfield_2_callback)(const void*);
  uint16_t (*external_battery_voltage_callback)(const void*);
  uint16_t (*max_battery_voltage_callback)(const void*);
  uint16_t (*min_battery_voltage_callback)(const void*);
  uint16_t (*max_cell_voltage_callback)(const void*);
  uint16_t (*max_cell_voltage_string_callback)(const void*);
  uint16_t (*max_cell_voltage_module_callback)(const void*);
  uint16_t (*min_cell_voltage_callback)(const void*);
  uint16_t (*min_cell_voltage_string_callback)(const void*);
  uint16_t (*min_cell_voltage_module_callback)(const void*);
  uint16_t (*average_cell_voltage_callback)(const void*);
  int16_t (*total_dc_current_callback)(const void*);
  uint16_t (*max_charge_current_callback)(const void*);
  uint16_t (*max_discharge_current_callback)(const void*);
  int16_t (*total_power_callback)(const void*);
  ReqInvState (*inverter_state_request_callback)(const void*);
  int16_t (*battery_power_request_callback)(const void*);
  SetOp (*operation_callback)(const void*);
  void (*set_operation_callback)(SetOp, void*);
  SetInvState (*inverter_state_callback)(const void*);
  void (*set_inverter_state_callback)(SetInvState, void*);
  uint16_t (*ah_rtg_sf_callback)(const void*);
  uint16_t (*wh_rtg_sf_callback)(const void*);
  uint16_t (*w_cha_dis_cha_max_sf_callback)(const void*);
  uint16_t (*dis_cha_rte_sf_callback)(const void*);
  uint16_t (*so_c_sf_callback)(const void*);
  uint16_t (*do_d_sf_callback)(const void*);
  uint16_t (*so_h_sf_callback)(const void*);
  uint16_t (*v_sf_callback)(const void*);
  uint16_t (*cell_v_sf_callback)(const void*);
  uint16_t (*a_sf_callback)(const void*);
  uint16_t (*a_max_sf_callback)(const void*);
  uint16_t (*w_sf_callback)(const void*);
} Model802CallbackAdapter;

typedef struct Model802StatefulAdapter {
  uint16_t nameplate_charge_capacity;
  uint16_t nameplate_energy_capacity;
  uint16_t nameplate_max_charge_rate;
  uint16_t nameplate_max_discharge_rate;
  uint16_t self_discharge_rate;
  uint16_t nameplate_max_so_c;
  uint16_t nameplate_min_so_c;
  uint16_t max_reserve_percent;
  uint16_t min_reserve_percent;
  uint16_t state_of_charge;
  uint16_t depth_of_discharge;
  uint16_t state_of_health;
  uint32_t cycle_count;
  ChaSt charge_status;
  LocRemCtl control_mode;
  uint16_t battery_heartbeat;
  uint16_t controller_heartbeat;
  uint16_t alarm_reset;
  Typ battery_type;
  State state_of_the_battery_bank;
  uint16_t vendor_battery_bank_state;
  uint32_t warranty_date;
  uint32_t battery_event_1_bitfield;
  uint32_t battery_event_2_bitfield;
  uint32_t vendor_event_bitfield_1;
  uint32_t vendor_event_bitfield_2;
  uint16_t external_battery_voltage;
  uint16_t max_battery_voltage;
  uint16_t min_battery_voltage;
  uint16_t max_cell_voltage;
  uint16_t max_cell_voltage_string;
  uint16_t max_cell_voltage_module;
  uint16_t min_cell_voltage;
  uint16_t min_cell_voltage_string;
  uint16_t min_cell_voltage_module;
  uint16_t average_cell_voltage;
  int16_t total_dc_current;
  uint16_t max_charge_current;
  uint16_t max_discharge_current;
  int16_t total_power;
  ReqInvState inverter_state_request;
  int16_t battery_power_request;
  SetOp operation;
  SetInvState inverter_state;
  uint16_t ah_rtg_sf;
  uint16_t wh_rtg_sf;
  uint16_t w_cha_dis_cha_max_sf;
  uint16_t dis_cha_rte_sf;
  uint16_t so_c_sf;
  uint16_t do_d_sf;
  uint16_t so_h_sf;
  uint16_t v_sf;
  uint16_t cell_v_sf;
  uint16_t a_sf;
  uint16_t a_max_sf;
  uint16_t w_sf;
} Model802StatefulAdapter;

typedef struct Model803CallbackAdapter {
  void *context;
  uint16_t (*string_count_callback)(const void*);
  uint16_t (*connected_string_count_callback)(const void*);
  int16_t (*max_module_temperature_callback)(const void*);
  uint16_t (*max_module_temperature_string_callback)(const void*);
  uint16_t (*max_module_temperature_module_callback)(const void*);
  int16_t (*min_module_temperature_callback)(const void*);
  uint16_t (*min_module_temperature_string_callback)(const void*);
  uint16_t (*min_module_temperature_module_callback)(const void*);
  int16_t (*average_module_temperature_callback)(const void*);
  uint16_t (*max_string_voltage_callback)(const void*);
  uint16_t (*max_string_voltage_string_callback)(const void*);
  uint16_t (*min_string_voltage_callback)(const void*);
  uint16_t (*min_string_voltage_string_callback)(const void*);
  uint16_t (*average_string_voltage_callback)(const void*);
  int16_t (*max_string_current_callback)(const void*);
  uint16_t (*max_string_current_string_callback)(const void*);
  int16_t (*min_string_current_callback)(const void*);
  uint16_t (*min_string_current_string_callback)(const void*);
  int16_t (*average_string_current_callback)(const void*);
  uint16_t (*battery_cell_balancing_count_callback)(const void*);
  uint16_t (*cell_v_sf_callback)(const void*);
  uint16_t (*mod_tmp_sf_callback)(const void*);
  uint16_t (*a_sf_callback)(const void*);
  uint16_t (*so_h_sf_callback)(const void*);
  uint16_t (*so_c_sf_callback)(const void*);
  uint16_t (*v_sf_callback)(const void*);
} Model803CallbackAdapter;

typedef struct Model803StatefulAdapter {
  uint16_t string_count;
  uint16_t connected_string_count;
  int16_t max_module_temperature;
  uint16_t max_module_temperature_string;
  uint16_t max_module_temperature_module;
  int16_t min_module_temperature;
  uint16_t min_module_temperature_string;
  uint16_t min_module_temperature_module;
  int16_t average_module_temperature;
  uint16_t max_string_voltage;
  uint16_t max_string_voltage_string;
  uint16_t min_string_voltage;
  uint16_t min_string_voltage_string;
  uint16_t average_string_voltage;
  int16_t max_string_current;
  uint16_t max_string_current_string;
  int16_t min_string_current;
  uint16_t min_string_current_string;
  int16_t average_string_current;
  uint16_t battery_cell_balancing_count;
  uint16_t cell_v_sf;
  uint16_t mod_tmp_sf;
  uint16_t a_sf;
  uint16_t so_h_sf;
  uint16_t so_c_sf;
  uint16_t v_sf;
} Model803StatefulAdapter;

typedef struct Model804CallbackAdapter {
  void *context;
  uint16_t (*string_index_callback)(const void*);
  uint16_t (*module_count_callback)(const void*);
  uint32_t (*string_status_callback)(const void*);
  ConFail (*connection_failure_reason_callback)(const void*);
  uint16_t (*string_cell_balancing_count_callback)(const void*);
  uint16_t (*string_state_of_charge_callback)(const void*);
  uint16_t (*string_depth_of_discharge_callback)(const void*);
  uint32_t (*string_cycle_count_callback)(const void*);
  uint16_t (*string_state_of_health_callback)(const void*);
  int16_t (*string_current_callback)(const void*);
  uint16_t (*string_voltage_callback)(const void*);
  uint16_t (*max_cell_voltage_callback)(const void*);
  uint16_t (*max_cell_voltage_module_callback)(const void*);
  uint16_t (*min_cell_voltage_callback)(const void*);
  uint16_t (*min_cell_voltage_module_callback)(const void*);
  uint16_t (*average_cell_voltage_callback)(const void*);
  int16_t (*max_module_temperature_callback)(const void*);
  uint16_t (*max_module_temperature_module_callback)(const void*);
  int16_t (*min_module_temperature_callback)(const void*);
  uint16_t (*min_module_temperature_module_callback)(const void*);
  int16_t (*average_module_temperature_callback)(const void*);
  uint32_t (*contactor_status_callback)(const void*);
  uint32_t (*string_event_1_callback)(const void*);
  uint32_t (*string_event_2_callback)(const void*);
  uint32_t (*vendor_event_bitfield_1_callback)(const void*);
  uint32_t (*vendor_event_bitfield_2_callback)(const void*);
  uint16_t (*enable_disable_string_callback)(const void*);
  void (*set_enable_disable_string_callback)(uint16_t, void*);
  SetCon (*connect_disconnect_string_callback)(const void*);
  void (*set_connect_disconnect_string_callback)(SetCon, void*);
  uint16_t (*so_c_sf_callback)(const void*);
  uint16_t (*so_h_sf_callback)(const void*);
  uint16_t (*do_d_sf_callback)(const void*);
  uint16_t (*a_sf_callback)(const void*);
  uint16_t (*v_sf_callback)(const void*);
  uint16_t (*cell_v_sf_callback)(const void*);
  uint16_t (*mod_tmp_sf_callback)(const void*);
} Model804CallbackAdapter;

typedef struct Model804StatefulAdapter {
  uint16_t string_index;
  uint16_t module_count;
  uint32_t string_status;
  ConFail connection_failure_reason;
  uint16_t string_cell_balancing_count;
  uint16_t string_state_of_charge;
  uint16_t string_depth_of_discharge;
  uint32_t string_cycle_count;
  uint16_t string_state_of_health;
  int16_t string_current;
  uint16_t string_voltage;
  uint16_t max_cell_voltage;
  uint16_t max_cell_voltage_module;
  uint16_t min_cell_voltage;
  uint16_t min_cell_voltage_module;
  uint16_t average_cell_voltage;
  int16_t max_module_temperature;
  uint16_t max_module_temperature_module;
  int16_t min_module_temperature;
  uint16_t min_module_temperature_module;
  int16_t average_module_temperature;
  uint32_t contactor_status;
  uint32_t string_event_1;
  uint32_t string_event_2;
  uint32_t vendor_event_bitfield_1;
  uint32_t vendor_event_bitfield_2;
  uint16_t enable_disable_string;
  SetCon connect_disconnect_string;
  uint16_t so_c_sf;
  uint16_t so_h_sf;
  uint16_t do_d_sf;
  uint16_t a_sf;
  uint16_t v_sf;
  uint16_t cell_v_sf;
  uint16_t mod_tmp_sf;
} Model804StatefulAdapter;

typedef struct Model805CallbackAdapter {
  void *context;
  uint16_t (*string_index_callback)(const void*);
  uint16_t (*module_index_callback)(const void*);
  uint16_t (*module_cell_count_callback)(const void*);
  uint16_t (*module_so_c_callback)(const void*);
  uint16_t (*depth_of_discharge_callback)(const void*);
  uint16_t (*module_so_h_callback)(const void*);
  uint32_t (*cycle_count_callback)(const void*);
  uint16_t (*module_voltage_callback)(const void*);
  uint16_t (*max_cell_voltage_callback)(const void*);
  uint16_t (*max_cell_voltage_cell_callback)(const void*);
  uint16_t (*min_cell_voltage_callback)(const void*);
  uint16_t (*min_cell_voltage_cell_callback)(const void*);
  uint16_t (*average_cell_voltage_callback)(const void*);
  int16_t (*max_cell_temperature_callback)(const void*);
  uint16_t (*max_cell_temperature_cell_callback)(const void*);
  int16_t (*min_cell_temperature_callback)(const void*);
  uint16_t (*min_cell_temperature_cell_callback)(const void*);
  int16_t (*average_cell_temperature_callback)(const void*);
  uint16_t (*balanced_cell_count_callback)(const void*);
  const char *(*serial_number_callback)(const void*);
  uint16_t (*so_c_sf_callback)(const void*);
  uint16_t (*so_h_sf_callback)(const void*);
  uint16_t (*do_d_sf_callback)(const void*);
  uint16_t (*v_sf_callback)(const void*);
  uint16_t (*cell_v_sf_callback)(const void*);
  uint16_t (*tmp_sf_callback)(const void*);
} Model805CallbackAdapter;

typedef struct Model805StatefulAdapter {
  uint16_t string_index;
  uint16_t module_index;
  uint16_t module_cell_count;
  uint16_t module_so_c;
  uint16_t depth_of_discharge;
  uint16_t module_so_h;
  uint32_t cycle_count;
  uint16_t module_voltage;
  uint16_t max_cell_voltage;
  uint16_t max_cell_voltage_cell;
  uint16_t min_cell_voltage;
  uint16_t min_cell_voltage_cell;
  uint16_t average_cell_voltage;
  int16_t max_cell_temperature;
  uint16_t max_cell_temperature_cell;
  int16_t min_cell_temperature;
  uint16_t min_cell_temperature_cell;
  int16_t average_cell_temperature;
  uint16_t balanced_cell_count;
  char serial_number[32];
  uint16_t so_c_sf;
  uint16_t so_h_sf;
  uint16_t do_d_sf;
  uint16_t v_sf;
  uint16_t cell_v_sf;
  uint16_t tmp_sf;
} Model805StatefulAdapter;

typedef struct Model806CallbackAdapter {
  void *context;
  uint16_t (*battery_points_to_be_determined_callback)(const void*);
} Model806CallbackAdapter;

typedef struct Model806StatefulAdapter {
  uint16_t battery_points_to_be_determined;
} Model806StatefulAdapter;

typedef struct Model807CallbackAdapter {
  void *context;
  uint16_t (*string_index_callback)(const void*);
  uint16_t (*module_count_callback)(const void*);
  uint16_t (*connected_module_count_callback)(const void*);
  uint16_t (*max_module_voltage_callback)(const void*);
  uint16_t (*max_module_voltage_module_callback)(const void*);
  uint16_t (*min_module_voltage_callback)(const void*);
  uint16_t (*min_module_voltage_module_callback)(const void*);
  uint16_t (*average_module_voltage_callback)(const void*);
  uint16_t (*max_cell_voltage_callback)(const void*);
  uint16_t (*max_cell_voltage_module_callback)(const void*);
  uint16_t (*max_cell_voltage_stack_callback)(const void*);
  uint16_t (*min_cell_voltage_callback)(const void*);
  uint16_t (*min_cell_voltage_module_callback)(const void*);
  uint16_t (*min_cell_voltage_stack_callback)(const void*);
  uint16_t (*average_cell_voltage_callback)(const void*);
  int16_t (*max_temperature_callback)(const void*);
  uint16_t (*max_temperature_module_callback)(const void*);
  int16_t (*min_temperature_callback)(const void*);
  uint16_t (*min_temperature_module_callback)(const void*);
  int16_t (*average_temperature_callback)(const void*);
  uint32_t (*string_event_1_callback)(const void*);
  uint32_t (*string_event_2_callback)(const void*);
  uint32_t (*vendor_event_bitfield_1_callback)(const void*);
  uint32_t (*vendor_event_bitfield_2_callback)(const void*);
  uint16_t (*mod_v_sf_callback)(const void*);
  uint16_t (*cell_v_sf_callback)(const void*);
  uint16_t (*tmp_sf_callback)(const void*);
  uint16_t (*so_c_sf_callback)(const void*);
  uint16_t (*ocv_sf_callback)(const void*);
} Model807CallbackAdapter;

typedef struct Model807StatefulAdapter {
  uint16_t string_index;
  uint16_t module_count;
  uint16_t connected_module_count;
  uint16_t max_module_voltage;
  uint16_t max_module_voltage_module;
  uint16_t min_module_voltage;
  uint16_t min_module_voltage_module;
  uint16_t average_module_voltage;
  uint16_t max_cell_voltage;
  uint16_t max_cell_voltage_module;
  uint16_t max_cell_voltage_stack;
  uint16_t min_cell_voltage;
  uint16_t min_cell_voltage_module;
  uint16_t min_cell_voltage_stack;
  uint16_t average_cell_voltage;
  int16_t max_temperature;
  uint16_t max_temperature_module;
  int16_t min_temperature;
  uint16_t min_temperature_module;
  int16_t average_temperature;
  uint32_t string_event_1;
  uint32_t string_event_2;
  uint32_t vendor_event_bitfield_1;
  uint32_t vendor_event_bitfield_2;
  uint16_t mod_v_sf;
  uint16_t cell_v_sf;
  uint16_t tmp_sf;
  uint16_t so_c_sf;
  uint16_t ocv_sf;
} Model807StatefulAdapter;

typedef struct Model808CallbackAdapter {
  void *context;
  uint16_t (*module_points_to_be_determined_callback)(const void*);
} Model808CallbackAdapter;

typedef struct Model808StatefulAdapter {
  uint16_t module_points_to_be_determined;
} Model808StatefulAdapter;

typedef struct Model809CallbackAdapter {
  void *context;
  uint16_t (*stack_points_to_be_determined_callback)(const void*);
} Model809CallbackAdapter;

typedef struct Model809StatefulAdapter {
  uint16_t stack_points_to_be_determined;
} Model809StatefulAdapter;

typedef struct Model63001CallbackAdapter {
  void *context;
  uint16_t (*sunssf_1_callback)(const void*);
  uint16_t (*sunssf_2_callback)(const void*);
  uint16_t (*sunssf_3_callback)(const void*);
  uint16_t (*sunssf_4_callback)(const void*);
  int16_t (*int16_1_callback)(const void*);
  int16_t (*int16_2_callback)(const void*);
  int16_t (*int16_3_callback)(const void*);
  int16_t (*int16_4_callback)(const void*);
  void (*set_int16_4_callback)(int16_t, void*);
  int16_t (*int16_5_callback)(const void*);
  int16_t (*int16_u_callback)(const void*);
  uint16_t (*uint16_1_callback)(const void*);
  uint16_t (*uint16_2_callback)(const void*);
  uint16_t (*uint16_3_callback)(const void*);
  uint16_t (*uint16_4_callback)(const void*);
  void (*set_uint16_4_callback)(uint16_t, void*);
  uint16_t (*uint16_5_callback)(const void*);
  uint16_t (*uint16_u_callback)(const void*);
  uint16_t (*acc16_callback)(const void*);
  uint16_t (*acc16_u_callback)(const void*);
  uint16_t (*enum16_callback)(const void*);
  uint16_t (*enum16_u_callback)(const void*);
  uint16_t (*bitfield16_callback)(const void*);
  uint16_t (*bitfield16_u_callback)(const void*);
  int32_t (*int32_1_callback)(const void*);
  int32_t (*int32_2_callback)(const void*);
  int32_t (*int32_3_callback)(const void*);
  void (*set_int32_3_callback)(int32_t, void*);
  int32_t (*int32_4_callback)(const void*);
  int32_t (*int32_5_callback)(const void*);
  int32_t (*int32_u_callback)(const void*);
  uint32_t (*uint32_1_callback)(const void*);
  uint32_t (*uint32_2_callback)(const void*);
  uint32_t (*uint32_3_callback)(const void*);
  void (*set_uint32_3_callback)(uint32_t, void*);
  uint32_t (*uint32_4_callback)(const void*);
  uint32_t (*uint32_5_callback)(const void*);
  uint32_t (*uint32_u_callback)(const void*);
  uint32_t (*acc32_callback)(const void*);
  uint32_t (*acc32_u_callback)(const void*);
  uint32_t (*enum32_callback)(const void*);
  uint32_t (*enum32_u_callback)(const void*);
  uint32_t (*bitfield32_callback)(const void*);
  uint32_t (*bitfield32_u_callback)(const void*);
  uint32_t (*ipaddr_callback)(const void*);
  void (*set_ipaddr_callback)(uint32_t, void*);
  uint32_t (*ipaddr_u_callback)(const void*);
  int64_t (*int64_callback)(const void*);
  void (*set_int64_callback)(int64_t, void*);
  int64_t (*int64_u_callback)(const void*);
  uint64_t (*acc64_callback)(const void*);
  uint64_t (*acc64_u_callback)(const void*);
  const uint16_t *(*ipv6addr_callback)(const void*);
  const uint16_t *(*ipv6addr_u_callback)(const void*);
  float (*float32_callback)(const void*);
  void (*set_float32_callback)(float, void*);
  float (*float32_u_callback)(const void*);
  const char *(*string_callback)(const void*);
  void (*set_string_callback)(const char*, void*);
  const char *(*string_u_callback)(const void*);
  uint16_t (*sunssf_5_callback)(const void*);
  uint16_t (*sunssf_6_callback)(const void*);
  uint16_t (*sunssf_7_callback)(const void*);
} Model63001CallbackAdapter;

typedef struct Model63001StatefulAdapter {
  uint16_t sunssf_1;
  uint16_t sunssf_2;
  uint16_t sunssf_3;
  uint16_t sunssf_4;
  int16_t int16_1;
  int16_t int16_2;
  int16_t int16_3;
  int16_t int16_4;
  int16_t int16_5;
  int16_t int16_u;
  uint16_t uint16_1;
  uint16_t uint16_2;
  uint16_t uint16_3;
  uint16_t uint16_4;
  uint16_t uint16_5;
  uint16_t uint16_u;
  uint16_t acc16;
  uint16_t acc16_u;
  uint16_t enum16;
  uint16_t enum16_u;
  uint16_t bitfield16;
  uint16_t bitfield16_u;
  int32_t int32_1;
  int32_t int32_2;
  int32_t int32_3;
  int32_t int32_4;
  int32_t int32_5;
  int32_t int32_u;
  uint32_t uint32_1;
  uint32_t uint32_2;
  uint32_t uint32_3;
  uint32_t uint32_4;
  uint32_t uint32_5;
  uint32_t uint32_u;
  uint32_t acc32;
  uint32_t acc32_u;
  uint32_t enum32;
  uint32_t enum32_u;
  uint32_t bitfield32;
  uint32_t bitfield32_u;
  uint32_t ipaddr;
  uint32_t ipaddr_u;
  int64_t int64;
  int64_t int64_u;
  uint64_t acc64;
  uint64_t acc64_u;
  uint16_t ipv6addr[8];
  uint16_t ipv6addr_u[8];
  float float32;
  float float32_u;
  char string[32];
  char string_u[32];
  uint16_t sunssf_5;
  uint16_t sunssf_6;
  uint16_t sunssf_7;
} Model63001StatefulAdapter;

typedef struct Model64001CallbackAdapter {
  void *context;
  uint16_t (*command_code_callback)(const void*);
  void (*set_command_code_callback)(uint16_t, void*);
  uint16_t (*hardware_revision_callback)(const void*);
  uint16_t (*rs_fw_revision_callback)(const void*);
  uint16_t (*os_fw_revision_callback)(const void*);
  const char *(*product_revision_callback)(const void*);
  uint16_t (*boot_count_callback)(const void*);
  uint16_t (*dip_switches_callback)(const void*);
  uint16_t (*num_detected_sensors_callback)(const void*);
  uint16_t (*num_communicating_sensors_callback)(const void*);
  uint16_t (*system_status_callback)(const void*);
  uint16_t (*system_configuration_callback)(const void*);
  uint16_t (*led_blink_threshold_callback)(const void*);
  uint16_t (*led_on_threshold_callback)(const void*);
  uint16_t (*reserved_callback)(const void*);
  const char *(*location_string_callback)(const void*);
  uint16_t (*sensor_1_unit_id_callback)(const void*);
  uint16_t (*sensor_1_address_callback)(const void*);
  uint16_t (*sensor_1_os_version_callback)(const void*);
  const char *(*sensor_1_product_version_callback)(const void*);
  const char *(*sensor_1_serial_num_callback)(const void*);
  uint16_t (*sensor_2_unit_id_callback)(const void*);
  uint16_t (*sensor_2_address_callback)(const void*);
  uint16_t (*sensor_2_os_version_callback)(const void*);
  const char *(*sensor_2_product_version_callback)(const void*);
  const char *(*sensor_2_serial_num_callback)(const void*);
  uint16_t (*sensor_3_unit_id_callback)(const void*);
  uint16_t (*sensor_3_address_callback)(const void*);
  uint16_t (*sensor_3_os_version_callback)(const void*);
  const char *(*sensor_3_product_version_callback)(const void*);
  const char *(*sensor_3_serial_num_callback)(const void*);
  uint16_t (*sensor_4_unit_id_callback)(const void*);
  uint16_t (*sensor_4_address_callback)(const void*);
  uint16_t (*sensor_4_os_version_callback)(const void*);
  const char *(*sensor_4_product_version_callback)(const void*);
  const char *(*sensor_4_serial_num_callback)(const void*);
} Model64001CallbackAdapter;

typedef struct Model64001StatefulAdapter {
  uint16_t command_code;
  uint16_t hardware_revision;
  uint16_t rs_fw_revision;
  uint16_t os_fw_revision;
  char product_revision[4];
  uint16_t boot_count;
  uint16_t dip_switches;
  uint16_t num_detected_sensors;
  uint16_t num_communicating_sensors;
  uint16_t system_status;
  uint16_t system_configuration;
  uint16_t led_blink_threshold;
  uint16_t led_on_threshold;
  uint16_t reserved;
  char location_string[32];
  uint16_t sensor_1_unit_id;
  uint16_t sensor_1_address;
  uint16_t sensor_1_os_version;
  char sensor_1_product_version[4];
  char sensor_1_serial_num[10];
  uint16_t sensor_2_unit_id;
  uint16_t sensor_2_address;
  uint16_t sensor_2_os_version;
  char sensor_2_product_version[4];
  char sensor_2_serial_num[10];
  uint16_t sensor_3_unit_id;
  uint16_t sensor_3_address;
  uint16_t sensor_3_os_version;
  char sensor_3_product_version[4];
  char sensor_3_serial_num[10];
  uint16_t sensor_4_unit_id;
  uint16_t sensor_4_address;
  uint16_t sensor_4_os_version;
  char sensor_4_product_version[4];
  char sensor_4_serial_num[10];
} Model64001StatefulAdapter;

typedef struct Model64020CallbackAdapter {
  void *context;
  int16_t (*aux_0_temperature_callback)(const void*);
  int16_t (*aux_1_temperature_callback)(const void*);
  int16_t (*aux_2_temperature_callback)(const void*);
  int16_t (*aux_3_temperature_callback)(const void*);
  int16_t (*aux_4_temperature_callback)(const void*);
  int16_t (*probe_temperature_callback)(const void*);
  int16_t (*main_temperature_callback)(const void*);
  uint16_t (*voltage_scale_factor_for_the_sensors_callback)(const void*);
  uint16_t (*current_scale_factor_for_the_sensors_callback)(const void*);
  uint16_t (*frequency_scale_factor_for_the_sensors_callback)(const void*);
  int16_t (*sensor1_voltage_callback)(const void*);
  int16_t (*sensor2_voltage_callback)(const void*);
  int16_t (*sensor3_voltage_callback)(const void*);
  int16_t (*sensor4_voltage_callback)(const void*);
  int16_t (*sensor5_voltage_callback)(const void*);
  int16_t (*sensor6_voltage_callback)(const void*);
  int16_t (*sensor7_voltage_callback)(const void*);
  int16_t (*sensor1_current_callback)(const void*);
  int16_t (*sensor2_current_callback)(const void*);
  int16_t (*sensor3_current_callback)(const void*);
  int16_t (*sensor4_current_callback)(const void*);
  int16_t (*sensor5_current_callback)(const void*);
  int16_t (*sensor6_current_callback)(const void*);
  int16_t (*sensor7_current_callback)(const void*);
  uint16_t (*sensor8_frequency_callback)(const void*);
  uint16_t (*relay_1_state_callback)(const void*);
  uint16_t (*relay_2_state_callback)(const void*);
  uint16_t (*relay_3_state_callback)(const void*);
  uint16_t (*reset_the_accumulators_callback)(const void*);
  uint16_t (*reset_the_system_callback)(const void*);
} Model64020CallbackAdapter;

typedef struct Model64020StatefulAdapter {
  int16_t aux_0_temperature;
  int16_t aux_1_temperature;
  int16_t aux_2_temperature;
  int16_t aux_3_temperature;
  int16_t aux_4_temperature;
  int16_t probe_temperature;
  int16_t main_temperature;
  uint16_t voltage_scale_factor_for_the_sensors;
  uint16_t current_scale_factor_for_the_sensors;
  uint16_t frequency_scale_factor_for_the_sensors;
  int16_t sensor1_voltage;
  int16_t sensor2_voltage;
  int16_t sensor3_voltage;
  int16_t sensor4_voltage;
  int16_t sensor5_voltage;
  int16_t sensor6_voltage;
  int16_t sensor7_voltage;
  int16_t sensor1_current;
  int16_t sensor2_current;
  int16_t sensor3_current;
  int16_t sensor4_current;
  int16_t sensor5_current;
  int16_t sensor6_current;
  int16_t sensor7_current;
  uint16_t sensor8_frequency;
  uint16_t relay_1_state;
  uint16_t relay_2_state;
  uint16_t relay_3_state;
  uint16_t reset_the_accumulators;
  uint16_t reset_the_system;
} Model64020StatefulAdapter;

typedef struct Model64101CallbackAdapter {
  void *context;
  uint16_t (*eltek_country_code_callback)(const void*);
  uint16_t (*eltek_feeding_phase_callback)(const void*);
  uint16_t (*eltek_apd_method_callback)(const void*);
  uint16_t (*eltek_apd_power_ref_callback)(const void*);
  uint16_t (*eltek_rps_method_callback)(const void*);
  uint16_t (*eltek_rps_q_ref_callback)(const void*);
  int16_t (*eltek_rps_cos_phi_ref_callback)(const void*);
} Model64101CallbackAdapter;

typedef struct Model64101StatefulAdapter {
  uint16_t eltek_country_code;
  uint16_t eltek_feeding_phase;
  uint16_t eltek_apd_method;
  uint16_t eltek_apd_power_ref;
  uint16_t eltek_rps_method;
  uint16_t eltek_rps_q_ref;
  int16_t eltek_rps_cos_phi_ref;
} Model64101StatefulAdapter;

typedef struct Model64111CallbackAdapter {
  void *context;
  uint16_t (*port_number_callback)(const void*);
  uint16_t (*v_sf_callback)(const void*);
  uint16_t (*a_sf_callback)(const void*);
  uint16_t (*p_sf_callback)(const void*);
  uint16_t (*ah_sf_callback)(const void*);
  uint16_t (*kwh_sf_callback)(const void*);
  uint16_t (*battery_voltage_callback)(const void*);
  uint16_t (*array_voltage_callback)(const void*);
  uint16_t (*output_current_callback)(const void*);
  uint16_t (*array_current_callback)(const void*);
  ChargerSt (*operating_state_callback)(const void*);
  uint16_t (*output_wattage_callback)(const void*);
  uint16_t (*today_s_minimum_battery_voltage_callback)(const void*);
  uint16_t (*today_s_maximum_battery_voltage_callback)(const void*);
  uint16_t (*voc_callback)(const void*);
  uint16_t (*today_s_maximum_voc_callback)(const void*);
  uint16_t (*today_s_k_wh_callback)(const void*);
  uint16_t (*today_s_ah_callback)(const void*);
  uint16_t (*lifetime_k_wh_callback)(const void*);
  uint16_t (*lifetime_k_ah_callback)(const void*);
  uint16_t (*lifetime_maximum_output_wattage_callback)(const void*);
  uint16_t (*lifetime_maximum_battery_voltage_callback)(const void*);
  uint16_t (*lifetime_maximum_voc_voltage_callback)(const void*);
} Model64111CallbackAdapter;

typedef struct Model64111StatefulAdapter {
  uint16_t port_number;
  uint16_t v_sf;
  uint16_t a_sf;
  uint16_t p_sf;
  uint16_t ah_sf;
  uint16_t kwh_sf;
  uint16_t battery_voltage;
  uint16_t array_voltage;
  uint16_t output_current;
  uint16_t array_current;
  ChargerSt operating_state;
  uint16_t output_wattage;
  uint16_t today_s_minimum_battery_voltage;
  uint16_t today_s_maximum_battery_voltage;
  uint16_t voc;
  uint16_t today_s_maximum_voc;
  uint16_t today_s_k_wh;
  uint16_t today_s_ah;
  uint16_t lifetime_k_wh;
  uint16_t lifetime_k_ah;
  uint16_t lifetime_maximum_output_wattage;
  uint16_t lifetime_maximum_battery_voltage;
  uint16_t lifetime_maximum_voc_voltage;
} Model64111StatefulAdapter;

typedef struct Model64112CallbackAdapter {
  void *context;
  uint16_t (*port_number_callback)(const void*);
  uint16_t (*v_sf_callback)(const void*);
  uint16_t (*c_sf_callback)(const void*);
  uint16_t (*h_sf_callback)(const void*);
  uint16_t (*p_sf_callback)(const void*);
  uint16_t (*ah_sf_callback)(const void*);
  uint16_t (*kwh_sf_callback)(const void*);
  uint16_t (*faults_callback)(const void*);
  uint16_t (*absorb_callback)(const void*);
  uint16_t (*absorb_time_callback)(const void*);
  uint16_t (*absorb_end_callback)(const void*);
  uint16_t (*rebulk_callback)(const void*);
  uint16_t (*float_callback)(const void*);
  uint16_t (*maximum_charge_callback)(const void*);
  uint16_t (*equalize_callback)(const void*);
  uint16_t (*equalize_time_callback)(const void*);
  uint16_t (*auto_equalize_interval_callback)(const void*);
  CcConfigMpptMode (*mppt_mode_callback)(const void*);
  CcConfigSweepWidth (*sweep_width_callback)(const void*);
  CcConfigSweepMax (*sweep_maximum_callback)(const void*);
  uint16_t (*u_pick_pwm_duty_cycle_callback)(const void*);
  CcConfigGridTie (*grid_tie_mode_callback)(const void*);
  CcConfigTempComp (*temp_comp_mode_callback)(const void*);
  uint16_t (*temp_comp_lower_limit_callback)(const void*);
  uint16_t (*temp_comp_upper_limit_callback)(const void*);
  CcConfigAutoRestart (*auto_restart_mode_callback)(const void*);
  uint16_t (*wakeup_voc_change_callback)(const void*);
  uint16_t (*snooze_mode_callback)(const void*);
  uint16_t (*wakeup_interval_callback)(const void*);
  CcConfigAuxMode (*aux_output_mode_callback)(const void*);
  CcConfigAuxControl (*aux_output_control_callback)(const void*);
  CcConfigAuxState (*aux_output_state_callback)(const void*);
  CcConfigAuxPolarity (*aux_output_polarity_callback)(const void*);
  uint16_t (*aux_low_battery_disconnect_callback)(const void*);
  uint16_t (*aux_low_battery_reconnect_callback)(const void*);
  uint16_t (*aux_low_battery_disconnect_delay_callback)(const void*);
  uint16_t (*aux_vent_fan_callback)(const void*);
  uint16_t (*aux_pv_trigger_callback)(const void*);
  uint16_t (*aux_pv_trigger_hold_time_callback)(const void*);
  uint16_t (*aux_night_light_threshold_callback)(const void*);
  uint16_t (*aux_night_light_on_time_callback)(const void*);
  uint16_t (*aux_night_light_on_hysteresis_callback)(const void*);
  uint16_t (*aux_night_light_off_hysteresis_callback)(const void*);
  uint16_t (*aux_error_output_low_battery_callback)(const void*);
  uint16_t (*aux_divert_hold_time_callback)(const void*);
  uint16_t (*aux_divert_delay_time_callback)(const void*);
  uint16_t (*aux_divert_relative_callback)(const void*);
  uint16_t (*aux_divert_hysteresis_callback)(const void*);
  uint16_t (*fm_cc_major_firmware_number_callback)(const void*);
  uint16_t (*fm_cc_mid_firmware_number_callback)(const void*);
  uint16_t (*fm_cc_minor_firmware_number_callback)(const void*);
  uint16_t (*set_data_log_day_offset_callback)(const void*);
  uint16_t (*current_data_log_day_offset_callback)(const void*);
  uint16_t (*data_log_daily_ah_callback)(const void*);
  uint16_t (*data_log_daily_k_wh_callback)(const void*);
  uint16_t (*data_log_daily_maximum_output_a_callback)(const void*);
  uint16_t (*data_log_daily_maximum_output_w_callback)(const void*);
  uint16_t (*data_log_daily_absorb_time_callback)(const void*);
  uint16_t (*data_log_daily_float_time_callback)(const void*);
  uint16_t (*data_log_daily_minimum_battery_callback)(const void*);
  uint16_t (*data_log_daily_maximum_battery_callback)(const void*);
  uint16_t (*data_log_daily_maximum_input_callback)(const void*);
  uint16_t (*data_log_clear_callback)(const void*);
  uint16_t (*data_log_clear_complement_callback)(const void*);
} Model64112CallbackAdapter;

typedef struct Model64112StatefulAdapter {
  uint16_t port_number;
  uint16_t v_sf;
  uint16_t c_sf;
  uint16_t h_sf;
  uint16_t p_sf;
  uint16_t ah_sf;
  uint16_t kwh_sf;
  uint16_t faults;
  uint16_t absorb;
  uint16_t absorb_time;
  uint16_t absorb_end;
  uint16_t rebulk;
  uint16_t float_;
  uint16_t maximum_charge;
  uint16_t equalize;
  uint16_t equalize_time;
  uint16_t auto_equalize_interval;
  CcConfigMpptMode mppt_mode;
  CcConfigSweepWidth sweep_width;
  CcConfigSweepMax sweep_maximum;
  uint16_t u_pick_pwm_duty_cycle;
  CcConfigGridTie grid_tie_mode;
  CcConfigTempComp temp_comp_mode;
  uint16_t temp_comp_lower_limit;
  uint16_t temp_comp_upper_limit;
  CcConfigAutoRestart auto_restart_mode;
  uint16_t wakeup_voc_change;
  uint16_t snooze_mode;
  uint16_t wakeup_interval;
  CcConfigAuxMode aux_output_mode;
  CcConfigAuxControl aux_output_control;
  CcConfigAuxState aux_output_state;
  CcConfigAuxPolarity aux_output_polarity;
  uint16_t aux_low_battery_disconnect;
  uint16_t aux_low_battery_reconnect;
  uint16_t aux_low_battery_disconnect_delay;
  uint16_t aux_vent_fan;
  uint16_t aux_pv_trigger;
  uint16_t aux_pv_trigger_hold_time;
  uint16_t aux_night_light_threshold;
  uint16_t aux_night_light_on_time;
  uint16_t aux_night_light_on_hysteresis;
  uint16_t aux_night_light_off_hysteresis;
  uint16_t aux_error_output_low_battery;
  uint16_t aux_divert_hold_time;
  uint16_t aux_divert_delay_time;
  uint16_t aux_divert_relative;
  uint16_t aux_divert_hysteresis;
  uint16_t fm_cc_major_firmware_number;
  uint16_t fm_cc_mid_firmware_number;
  uint16_t fm_cc_minor_firmware_number;
  uint16_t set_data_log_day_offset;
  uint16_t current_data_log_day_offset;
  uint16_t data_log_daily_ah;
  uint16_t data_log_daily_k_wh;
  uint16_t data_log_daily_maximum_output_a;
  uint16_t data_log_daily_maximum_output_w;
  uint16_t data_log_daily_absorb_time;
  uint16_t data_log_daily_float_time;
  uint16_t data_log_daily_minimum_battery;
  uint16_t data_log_daily_maximum_battery;
  uint16_t data_log_daily_maximum_input;
  uint16_t data_log_clear;
  uint16_t data_log_clear_complement;
} Model64112StatefulAdapter;

typedef struct Model64410CallbackAdapter {
  void *context;
  uint16_t (*maximum_voltage_callback)(const void*);
  void (*set_maximum_voltage_callback)(uint16_t, void*);
  uint16_t (*maximum_power_callback)(const void*);
  void (*set_maximum_power_callback)(uint16_t, void*);
  uint16_t (*maximum_current_callback)(const void*);
  void (*set_maximum_current_callback)(uint16_t, void*);
  Mode (*cv_or_cc_mode_callback)(const void*);
  void (*set_cv_or_cc_mode_callback)(Mode, void*);
  Ena (*power_on_off_callback)(const void*);
  void (*set_power_on_off_callback)(Ena, void*);
  Reset (*reset_device_callback)(const void*);
  void (*set_reset_device_callback)(Reset, void*);
  uint16_t (*voltage_setpoint_callback)(const void*);
  void (*set_voltage_setpoint_callback)(uint16_t, void*);
  uint16_t (*power_setpoint_callback)(const void*);
  void (*set_power_setpoint_callback)(uint16_t, void*);
  uint16_t (*current_setpoint_callback)(const void*);
  void (*set_current_setpoint_callback)(uint16_t, void*);
  En50530 (*en50530_mode_callback)(const void*);
  void (*set_en50530_mode_callback)(En50530, void*);
  uint16_t (*en50530_mpp_voltage_callback)(const void*);
  void (*set_en50530_mpp_voltage_callback)(uint16_t, void*);
  uint16_t (*en50530_mpp_power_callback)(const void*);
  void (*set_en50530_mpp_power_callback)(uint16_t, void*);
  uint16_t (*irradiance_setpoint_callback)(const void*);
  void (*set_irradiance_setpoint_callback)(uint16_t, void*);
  uint16_t (*voltage_slew_rate_callback)(const void*);
  void (*set_voltage_slew_rate_callback)(uint16_t, void*);
  uint16_t (*power_slew_rate_callback)(const void*);
  void (*set_power_slew_rate_callback)(uint16_t, void*);
  uint16_t (*current_slew_rate_callback)(const void*);
  void (*set_current_slew_rate_callback)(uint16_t, void*);
  EnaProf (*enable_profile_callback)(const void*);
  void (*set_enable_profile_callback)(EnaProf, void*);
  uint16_t (*profile_adoption_request_callback)(const void*);
  void (*set_profile_adoption_request_callback)(uint16_t, void*);
  AdptProfRslt (*adopt_profile_result_callback)(const void*);
  int32_t (*measured_voltage_callback)(const void*);
  int32_t (*measured_power_callback)(const void*);
  int32_t (*measured_current_callback)(const void*);
  const char *(*errors_callback)(const void*);
  uint16_t (*number_of_points_callback)(const void*);
  uint16_t (*stored_profile_count_callback)(const void*);
  uint16_t (*power_scale_factor_callback)(const void*);
  void (*set_power_scale_factor_callback)(uint16_t, void*);
  uint16_t (*voltage_scale_factor_callback)(const void*);
  void (*set_voltage_scale_factor_callback)(uint16_t, void*);
  uint16_t (*current_scale_factor_callback)(const void*);
  void (*set_current_scale_factor_callback)(uint16_t, void*);
  uint16_t (*irradiance_scale_factor_callback)(const void*);
  void (*set_irradiance_scale_factor_callback)(uint16_t, void*);
  uint16_t (*time_scale_factor_callback)(const void*);
  void (*set_time_scale_factor_callback)(uint16_t, void*);
  uint16_t (*voltage_slew_rate_scale_factor_callback)(const void*);
  void (*set_voltage_slew_rate_scale_factor_callback)(uint16_t, void*);
  uint16_t (*power_slew_rate_scale_factor_callback)(const void*);
  void (*set_power_slew_rate_scale_factor_callback)(uint16_t, void*);
  uint16_t (*current_slew_rate_scale_factor_callback)(const void*);
  void (*set_current_slew_rate_scale_factor_callback)(uint16_t, void*);
  uint16_t (*percent_scale_factor_callback)(const void*);
  void (*set_percent_scale_factor_callback)(uint16_t, void*);
} Model64410CallbackAdapter;

typedef struct Model64410StatefulAdapter {
  uint16_t maximum_voltage;
  uint16_t maximum_power;
  uint16_t maximum_current;
  Mode cv_or_cc_mode;
  Ena power_on_off;
  Reset reset_device;
  uint16_t voltage_setpoint;
  uint16_t power_setpoint;
  uint16_t current_setpoint;
  En50530 en50530_mode;
  uint16_t en50530_mpp_voltage;
  uint16_t en50530_mpp_power;
  uint16_t irradiance_setpoint;
  uint16_t voltage_slew_rate;
  uint16_t power_slew_rate;
  uint16_t current_slew_rate;
  EnaProf enable_profile;
  uint16_t profile_adoption_request;
  AdptProfRslt adopt_profile_result;
  int32_t measured_voltage;
  int32_t measured_power;
  int32_t measured_current;
  char errors[64];
  uint16_t number_of_points;
  uint16_t stored_profile_count;
  uint16_t power_scale_factor;
  uint16_t voltage_scale_factor;
  uint16_t current_scale_factor;
  uint16_t irradiance_scale_factor;
  uint16_t time_scale_factor;
  uint16_t voltage_slew_rate_scale_factor;
  uint16_t power_slew_rate_scale_factor;
  uint16_t current_slew_rate_scale_factor;
  uint16_t percent_scale_factor;
} Model64410StatefulAdapter;

typedef struct Model64411CallbackAdapter {
  void *context;
  uint16_t (*active_phases_callback)(const void*);
  void (*set_active_phases_callback)(uint16_t, void*);
  uint16_t (*phase_angle_callback)(const void*);
  void (*set_phase_angle_callback)(uint16_t, void*);
  uint16_t (*nominal_voltage_callback)(const void*);
  void (*set_nominal_voltage_callback)(uint16_t, void*);
  uint16_t (*maximum_voltage_callback)(const void*);
  void (*set_maximum_voltage_callback)(uint16_t, void*);
  uint16_t (*maximum_current_callback)(const void*);
  void (*set_maximum_current_callback)(uint16_t, void*);
  uint16_t (*frequency_callback)(const void*);
  void (*set_frequency_callback)(uint16_t, void*);
  Output (*output_state_callback)(const void*);
  void (*set_output_state_callback)(Output, void*);
  Relay (*relay_state_callback)(const void*);
  void (*set_relay_state_callback)(Relay, void*);
  Regen (*regeneration_state_callback)(const void*);
  void (*set_regeneration_state_callback)(Regen, void*);
  uint16_t (*voltage_setpoint_callback)(const void*);
  void (*set_voltage_setpoint_callback)(uint16_t, void*);
  uint16_t (*voltage_setpoint_phase_a_callback)(const void*);
  void (*set_voltage_setpoint_phase_a_callback)(uint16_t, void*);
  uint16_t (*voltage_setpoint_phase_b_callback)(const void*);
  void (*set_voltage_setpoint_phase_b_callback)(uint16_t, void*);
  uint16_t (*voltage_setpoint_phase_c_callback)(const void*);
  void (*set_voltage_setpoint_phase_c_callback)(uint16_t, void*);
  uint16_t (*frequency_slew_rate_callback)(const void*);
  void (*set_frequency_slew_rate_callback)(uint16_t, void*);
  uint16_t (*voltage_slew_rate_callback)(const void*);
  void (*set_voltage_slew_rate_callback)(uint16_t, void*);
  int32_t (*measured_voltage_phase_a_callback)(const void*);
  int32_t (*measured_voltage_phase_b_callback)(const void*);
  int32_t (*measured_voltage_phase_c_callback)(const void*);
  int32_t (*measured_frequency_callback)(const void*);
  int32_t (*measured_current_phase_a_callback)(const void*);
  int32_t (*measured_current_phase_b_callback)(const void*);
  int32_t (*measured_current_phase_c_callback)(const void*);
  const char *(*voltage_harmonics_phase_a_callback)(const void*);
  const char *(*voltage_harmonics_phase_b_callback)(const void*);
  const char *(*voltage_harmonics_phase_c_callback)(const void*);
  const char *(*current_harmonics_phase_a_callback)(const void*);
  const char *(*current_harmonics_phase_b_callback)(const void*);
  const char *(*current_harmonics_phase_c_callback)(const void*);
  const char *(*current_interharmonics_phase_a_callback)(const void*);
  const char *(*current_interharmonics_phase_b_callback)(const void*);
  const char *(*current_interharmonics_phase_c_callback)(const void*);
  uint16_t (*voltage_thd_phase_a_callback)(const void*);
  uint16_t (*voltage_thd_phase_b_callback)(const void*);
  uint16_t (*voltage_thd_phase_c_callback)(const void*);
  uint16_t (*current_thd_phase_a_callback)(const void*);
  uint16_t (*current_thd_phase_b_callback)(const void*);
  uint16_t (*current_thd_phase_c_callback)(const void*);
  EnaProf (*enable_profile_callback)(const void*);
  void (*set_enable_profile_callback)(EnaProf, void*);
  ProfRslt (*profile_result_callback)(const void*);
  uint16_t (*stored_profile_count_callback)(const void*);
  uint16_t (*max_profile_point_count_callback)(const void*);
  uint16_t (*voltage_scale_factor_callback)(const void*);
  uint16_t (*current_scale_factor_callback)(const void*);
  uint16_t (*time_scale_factor_callback)(const void*);
  uint16_t (*frequency_scale_factor_callback)(const void*);
  uint16_t (*frequency_slew_rate_scale_factor_callback)(const void*);
  uint16_t (*voltage_slew_rate_scale_factor_callback)(const void*);
  uint16_t (*thd_scale_factor_callback)(const void*);
} Model64411CallbackAdapter;

typedef struct Model64411StatefulAdapter {
  uint16_t active_phases;
  uint16_t phase_angle;
  uint16_t nominal_voltage;
  uint16_t maximum_voltage;
  uint16_t maximum_current;
  uint16_t frequency;
  Output output_state;
  Relay relay_state;
  Regen regeneration_state;
  uint16_t voltage_setpoint;
  uint16_t voltage_setpoint_phase_a;
  uint16_t voltage_setpoint_phase_b;
  uint16_t voltage_setpoint_phase_c;
  uint16_t frequency_slew_rate;
  uint16_t voltage_slew_rate;
  int32_t measured_voltage_phase_a;
  int32_t measured_voltage_phase_b;
  int32_t measured_voltage_phase_c;
  int32_t measured_frequency;
  int32_t measured_current_phase_a;
  int32_t measured_current_phase_b;
  int32_t measured_current_phase_c;
  char voltage_harmonics_phase_a[300];
  char voltage_harmonics_phase_b[300];
  char voltage_harmonics_phase_c[300];
  char current_harmonics_phase_a[300];
  char current_harmonics_phase_b[300];
  char current_harmonics_phase_c[300];
  char current_interharmonics_phase_a[300];
  char current_interharmonics_phase_b[300];
  char current_interharmonics_phase_c[300];
  uint16_t voltage_thd_phase_a;
  uint16_t voltage_thd_phase_b;
  uint16_t voltage_thd_phase_c;
  uint16_t current_thd_phase_a;
  uint16_t current_thd_phase_b;
  uint16_t current_thd_phase_c;
  EnaProf enable_profile;
  ProfRslt profile_result;
  uint16_t stored_profile_count;
  uint16_t max_profile_point_count;
  uint16_t voltage_scale_factor;
  uint16_t current_scale_factor;
  uint16_t time_scale_factor;
  uint16_t frequency_scale_factor;
  uint16_t frequency_slew_rate_scale_factor;
  uint16_t voltage_slew_rate_scale_factor;
  uint16_t thd_scale_factor;
} Model64411StatefulAdapter;

typedef struct Model64412CallbackAdapter {
  void *context;
  DaManipulation (*da_manipulation_callback)(const void*);
  void (*set_da_manipulation_callback)(DaManipulation, void*);
  FalsifyDeviceIdentity (*falsify_device_identity_callback)(const void*);
  void (*set_falsify_device_identity_callback)(FalsifyDeviceIdentity, void*);
  MeasPAlwaysNameplate (*meas_p_always_nameplate_callback)(const void*);
  void (*set_meas_p_always_nameplate_callback)(MeasPAlwaysNameplate, void*);
  MeasQAlwaysMinimum (*meas_q_always_minimum_callback)(const void*);
  void (*set_meas_q_always_minimum_callback)(MeasQAlwaysMinimum, void*);
  MeasQAlwaysMaximum (*meas_q_always_maximum_callback)(const void*);
  void (*set_meas_q_always_maximum_callback)(MeasQAlwaysMaximum, void*);
  MeasQAlwaysZero (*meas_q_always_zero_callback)(const void*);
  void (*set_meas_q_always_zero_callback)(MeasQAlwaysZero, void*);
  MeasZeroP (*meas_zero_p_callback)(const void*);
  void (*set_meas_zero_p_callback)(MeasZeroP, void*);
  MeasInvertQ (*meas_invert_q_callback)(const void*);
  void (*set_meas_invert_q_callback)(MeasInvertQ, void*);
  MeasLowV (*meas_low_v_callback)(const void*);
  void (*set_meas_low_v_callback)(MeasLowV, void*);
  MeasHighV (*meas_high_v_callback)(const void*);
  void (*set_meas_high_v_callback)(MeasHighV, void*);
  MeasLowL1v (*meas_low_l1_v_callback)(const void*);
  void (*set_meas_low_l1_v_callback)(MeasLowL1v, void*);
  MeasHighL1v (*meas_high_l1_v_callback)(const void*);
  void (*set_meas_high_l1_v_callback)(MeasHighL1v, void*);
  MeasLowF (*meas_low_f_callback)(const void*);
  void (*set_meas_low_f_callback)(MeasLowF, void*);
  MeasHighF (*meas_high_f_callback)(const void*);
  void (*set_meas_high_f_callback)(MeasHighF, void*);
  MeasLowAmps (*meas_low_amps_callback)(const void*);
  void (*set_meas_low_amps_callback)(MeasLowAmps, void*);
  MeasHighAmps (*meas_high_amps_callback)(const void*);
  void (*set_meas_high_amps_callback)(MeasHighAmps, void*);
  MeasHighS (*meas_high_s_callback)(const void*);
  void (*set_meas_high_s_callback)(MeasHighS, void*);
  MeasLowS (*meas_low_s_callback)(const void*);
  void (*set_meas_low_s_callback)(MeasLowS, void*);
  MeasHighQ (*meas_high_q_callback)(const void*);
  void (*set_meas_high_q_callback)(MeasHighQ, void*);
  MeasLowQ (*meas_low_q_callback)(const void*);
  void (*set_meas_low_q_callback)(MeasLowQ, void*);
  MeasLowPf (*meas_low_pf_callback)(const void*);
  void (*set_meas_low_pf_callback)(MeasLowPf, void*);
  MeasLowReversedPf (*meas_low_reversed_pf_callback)(const void*);
  void (*set_meas_low_reversed_pf_callback)(MeasLowReversedPf, void*);
  NameplateHighP (*nameplate_high_p_callback)(const void*);
  void (*set_nameplate_high_p_callback)(NameplateHighP, void*);
  NameplateLowP (*nameplate_low_p_callback)(const void*);
  void (*set_nameplate_low_p_callback)(NameplateLowP, void*);
  NameplateHighS (*nameplate_high_s_callback)(const void*);
  void (*set_nameplate_high_s_callback)(NameplateHighS, void*);
  NameplateLowS (*nameplate_low_s_callback)(const void*);
  void (*set_nameplate_low_s_callback)(NameplateLowS, void*);
  NameplateHighQ (*nameplate_high_q_callback)(const void*);
  void (*set_nameplate_high_q_callback)(NameplateHighQ, void*);
  NameplateLowQ (*nameplate_low_q_callback)(const void*);
  void (*set_nameplate_low_q_callback)(NameplateLowQ, void*);
  NameplateHighNomV (*nameplate_high_nom_v_callback)(const void*);
  void (*set_nameplate_high_nom_v_callback)(NameplateHighNomV, void*);
  NameplateLowNomV (*nameplate_low_nom_v_callback)(const void*);
  void (*set_nameplate_low_nom_v_callback)(NameplateLowNomV, void*);
  NameplateLowAmps (*nameplate_low_amps_callback)(const void*);
  void (*set_nameplate_low_amps_callback)(NameplateLowAmps, void*);
  NameplateLowVarmaxinj (*nameplate_low_varmaxinj_callback)(const void*);
  void (*set_nameplate_low_varmaxinj_callback)(NameplateLowVarmaxinj, void*);
  NameplateLowVarmaxabs (*nameplate_low_varmaxabs_callback)(const void*);
  void (*set_nameplate_low_varmaxabs_callback)(NameplateLowVarmaxabs, void*);
  NameplateLowPf (*nameplate_low_pf_callback)(const void*);
  void (*set_nameplate_low_pf_callback)(NameplateLowPf, void*);
  SettingsHighNomV (*settings_high_nom_v_callback)(const void*);
  void (*set_settings_high_nom_v_callback)(SettingsHighNomV, void*);
  SettingsLowAmps (*settings_low_amps_callback)(const void*);
  void (*set_settings_low_amps_callback)(SettingsLowAmps, void*);
  SettingsHighP (*settings_high_p_callback)(const void*);
  void (*set_settings_high_p_callback)(SettingsHighP, void*);
  SettingsLowP (*settings_low_p_callback)(const void*);
  void (*set_settings_low_p_callback)(SettingsLowP, void*);
  SettingsHighVaMax (*settings_high_va_max_callback)(const void*);
  void (*set_settings_high_va_max_callback)(SettingsHighVaMax, void*);
  SettingsHighVarmaxinj (*settings_high_varmaxinj_callback)(const void*);
  void (*set_settings_high_varmaxinj_callback)(SettingsHighVarmaxinj, void*);
  SettingsHighVarmaxabs (*settings_high_varmaxabs_callback)(const void*);
  void (*set_settings_high_varmaxabs_callback)(SettingsHighVarmaxabs, void*);
  ChangeCommonModelId (*change_common_model_id_callback)(const void*);
  void (*set_change_common_model_id_callback)(ChangeCommonModelId, void*);
  ChangeCommonModelLength (*change_common_model_length_callback)(const void*);
  void (*set_change_common_model_length_callback)(ChangeCommonModelLength, void*);
} Model64412CallbackAdapter;

typedef struct Model64412StatefulAdapter {
  DaManipulation da_manipulation;
  FalsifyDeviceIdentity falsify_device_identity;
  MeasPAlwaysNameplate meas_p_always_nameplate;
  MeasQAlwaysMinimum meas_q_always_minimum;
  MeasQAlwaysMaximum meas_q_always_maximum;
  MeasQAlwaysZero meas_q_always_zero;
  MeasZeroP meas_zero_p;
  MeasInvertQ meas_invert_q;
  MeasLowV meas_low_v;
  MeasHighV meas_high_v;
  MeasLowL1v meas_low_l1_v;
  MeasHighL1v meas_high_l1_v;
  MeasLowF meas_low_f;
  MeasHighF meas_high_f;
  MeasLowAmps meas_low_amps;
  MeasHighAmps meas_high_amps;
  MeasHighS meas_high_s;
  MeasLowS meas_low_s;
  MeasHighQ meas_high_q;
  MeasLowQ meas_low_q;
  MeasLowPf meas_low_pf;
  MeasLowReversedPf meas_low_reversed_pf;
  NameplateHighP nameplate_high_p;
  NameplateLowP nameplate_low_p;
  NameplateHighS nameplate_high_s;
  NameplateLowS nameplate_low_s;
  NameplateHighQ nameplate_high_q;
  NameplateLowQ nameplate_low_q;
  NameplateHighNomV nameplate_high_nom_v;
  NameplateLowNomV nameplate_low_nom_v;
  NameplateLowAmps nameplate_low_amps;
  NameplateLowVarmaxinj nameplate_low_varmaxinj;
  NameplateLowVarmaxabs nameplate_low_varmaxabs;
  NameplateLowPf nameplate_low_pf;
  SettingsHighNomV settings_high_nom_v;
  SettingsLowAmps settings_low_amps;
  SettingsHighP settings_high_p;
  SettingsLowP settings_low_p;
  SettingsHighVaMax settings_high_va_max;
  SettingsHighVarmaxinj settings_high_varmaxinj;
  SettingsHighVarmaxabs settings_high_varmaxabs;
  ChangeCommonModelId change_common_model_id;
  ChangeCommonModelLength change_common_model_length;
} Model64412StatefulAdapter;

typedef struct Model64413CallbackAdapter {
  void *context;
  uint16_t (*iv_length_callback)(const void*);
  uint16_t (*poa_irradiance_callback)(const void*);
  uint16_t (*irr_sf_callback)(const void*);
} Model64413CallbackAdapter;

typedef struct Model64413StatefulAdapter {
  uint16_t iv_length;
  uint16_t poa_irradiance;
  uint16_t irr_sf;
} Model64413StatefulAdapter;

typedef struct Model64414CallbackAdapter {
  void *context;
  const char *(*time_offset_callback)(const void*);
  float (*temperature_callback)(const void*);
  void (*set_temperature_callback)(float, void*);
  const char *(*grid_model_source_callback)(const void*);
  void (*set_grid_model_source_callback)(const char*, void*);
  const char *(*irradiance_model_source_callback)(const void*);
  void (*set_irradiance_model_source_callback)(const char*, void*);
  float (*irradiance_callback)(const void*);
  void (*set_irradiance_callback)(float, void*);
  float (*grid_voltage_a_callback)(const void*);
  void (*set_grid_voltage_a_callback)(float, void*);
  float (*grid_voltage_b_callback)(const void*);
  void (*set_grid_voltage_b_callback)(float, void*);
  float (*grid_voltage_c_callback)(const void*);
  void (*set_grid_voltage_c_callback)(float, void*);
  float (*grid_frequency_callback)(const void*);
  void (*set_grid_frequency_callback)(float, void*);
} Model64414CallbackAdapter;

typedef struct Model64414StatefulAdapter {
  char time_offset[20];
  float temperature;
  char grid_model_source[64];
  char irradiance_model_source[64];
  float irradiance;
  float grid_voltage_a;
  float grid_voltage_b;
  float grid_voltage_c;
  float grid_frequency;
} Model64414StatefulAdapter;

typedef struct Model64415CallbackAdapter {
  void *context;
  LogEventEna (*log_event_mode_enable_callback)(const void*);
  void (*set_log_event_mode_enable_callback)(LogEventEna, void*);
  HttpMsg (*http_message_mode_enable_callback)(const void*);
  void (*set_http_message_mode_enable_callback)(HttpMsg, void*);
  Comm004Cert (*comm_004_certificate_callback)(const void*);
  void (*set_comm_004_certificate_callback)(Comm004Cert, void*);
  const char *(*subscribed_resource_url_callback)(const void*);
  void (*set_subscribed_resource_url_callback)(const char*, void*);
  SubscriptionEna (*subscribtion_enable_callback)(const void*);
  void (*set_subscribtion_enable_callback)(SubscriptionEna, void*);
} Model64415CallbackAdapter;

typedef struct Model64415StatefulAdapter {
  LogEventEna log_event_mode_enable;
  HttpMsg http_message_mode_enable;
  Comm004Cert comm_004_certificate;
  char subscribed_resource_url[128];
  SubscriptionEna subscribtion_enable;
} Model64415StatefulAdapter;

typedef struct SunspecExternalAdapters {
  const struct Model1CallbackAdapter *model_1_callback_adapter;
  const struct Model1StatefulAdapter *model_1_stateful_adapter;
  const struct Model2CallbackAdapter *model_2_callback_adapter;
  const struct Model2StatefulAdapter *model_2_stateful_adapter;
  const struct Model3CallbackAdapter *model_3_callback_adapter;
  const struct Model3StatefulAdapter *model_3_stateful_adapter;
  const struct Model4CallbackAdapter *model_4_callback_adapter;
  const struct Model4StatefulAdapter *model_4_stateful_adapter;
  const struct Model5CallbackAdapter *model_5_callback_adapter;
  const struct Model5StatefulAdapter *model_5_stateful_adapter;
  const struct Model6CallbackAdapter *model_6_callback_adapter;
  const struct Model6StatefulAdapter *model_6_stateful_adapter;
  const struct Model7CallbackAdapter *model_7_callback_adapter;
  const struct Model7StatefulAdapter *model_7_stateful_adapter;
  const struct Model8CallbackAdapter *model_8_callback_adapter;
  const struct Model8StatefulAdapter *model_8_stateful_adapter;
  const struct Model10CallbackAdapter *model_10_callback_adapter;
  const struct Model10StatefulAdapter *model_10_stateful_adapter;
  const struct Model11CallbackAdapter *model_11_callback_adapter;
  const struct Model11StatefulAdapter *model_11_stateful_adapter;
  const struct Model12CallbackAdapter *model_12_callback_adapter;
  const struct Model12StatefulAdapter *model_12_stateful_adapter;
  const struct Model13CallbackAdapter *model_13_callback_adapter;
  const struct Model13StatefulAdapter *model_13_stateful_adapter;
  const struct Model15CallbackAdapter *model_15_callback_adapter;
  const struct Model15StatefulAdapter *model_15_stateful_adapter;
  const struct Model16CallbackAdapter *model_16_callback_adapter;
  const struct Model16StatefulAdapter *model_16_stateful_adapter;
  const struct Model17CallbackAdapter *model_17_callback_adapter;
  const struct Model17StatefulAdapter *model_17_stateful_adapter;
  const struct Model18CallbackAdapter *model_18_callback_adapter;
  const struct Model18StatefulAdapter *model_18_stateful_adapter;
  const struct Model19CallbackAdapter *model_19_callback_adapter;
  const struct Model19StatefulAdapter *model_19_stateful_adapter;
  const struct Model101CallbackAdapter *model_101_callback_adapter;
  const struct Model101StatefulAdapter *model_101_stateful_adapter;
  const struct Model102CallbackAdapter *model_102_callback_adapter;
  const struct Model102StatefulAdapter *model_102_stateful_adapter;
  const struct Model103CallbackAdapter *model_103_callback_adapter;
  const struct Model103StatefulAdapter *model_103_stateful_adapter;
  const struct Model111CallbackAdapter *model_111_callback_adapter;
  const struct Model111StatefulAdapter *model_111_stateful_adapter;
  const struct Model112CallbackAdapter *model_112_callback_adapter;
  const struct Model112StatefulAdapter *model_112_stateful_adapter;
  const struct Model113CallbackAdapter *model_113_callback_adapter;
  const struct Model113StatefulAdapter *model_113_stateful_adapter;
  const struct Model120CallbackAdapter *model_120_callback_adapter;
  const struct Model120StatefulAdapter *model_120_stateful_adapter;
  const struct Model121CallbackAdapter *model_121_callback_adapter;
  const struct Model121StatefulAdapter *model_121_stateful_adapter;
  const struct Model122CallbackAdapter *model_122_callback_adapter;
  const struct Model122StatefulAdapter *model_122_stateful_adapter;
  const struct Model123CallbackAdapter *model_123_callback_adapter;
  const struct Model123StatefulAdapter *model_123_stateful_adapter;
  const struct Model124CallbackAdapter *model_124_callback_adapter;
  const struct Model124StatefulAdapter *model_124_stateful_adapter;
  const struct Model125CallbackAdapter *model_125_callback_adapter;
  const struct Model125StatefulAdapter *model_125_stateful_adapter;
  const struct Model126CallbackAdapter *model_126_callback_adapter;
  const struct Model126StatefulAdapter *model_126_stateful_adapter;
  const struct Model127CallbackAdapter *model_127_callback_adapter;
  const struct Model127StatefulAdapter *model_127_stateful_adapter;
  const struct Model128CallbackAdapter *model_128_callback_adapter;
  const struct Model128StatefulAdapter *model_128_stateful_adapter;
  const struct Model129CallbackAdapter *model_129_callback_adapter;
  const struct Model129StatefulAdapter *model_129_stateful_adapter;
  const struct Model130CallbackAdapter *model_130_callback_adapter;
  const struct Model130StatefulAdapter *model_130_stateful_adapter;
  const struct Model131CallbackAdapter *model_131_callback_adapter;
  const struct Model131StatefulAdapter *model_131_stateful_adapter;
  const struct Model132CallbackAdapter *model_132_callback_adapter;
  const struct Model132StatefulAdapter *model_132_stateful_adapter;
  const struct Model133CallbackAdapter *model_133_callback_adapter;
  const struct Model133StatefulAdapter *model_133_stateful_adapter;
  const struct Model134CallbackAdapter *model_134_callback_adapter;
  const struct Model134StatefulAdapter *model_134_stateful_adapter;
  const struct Model135CallbackAdapter *model_135_callback_adapter;
  const struct Model135StatefulAdapter *model_135_stateful_adapter;
  const struct Model136CallbackAdapter *model_136_callback_adapter;
  const struct Model136StatefulAdapter *model_136_stateful_adapter;
  const struct Model137CallbackAdapter *model_137_callback_adapter;
  const struct Model137StatefulAdapter *model_137_stateful_adapter;
  const struct Model138CallbackAdapter *model_138_callback_adapter;
  const struct Model138StatefulAdapter *model_138_stateful_adapter;
  const struct Model139CallbackAdapter *model_139_callback_adapter;
  const struct Model139StatefulAdapter *model_139_stateful_adapter;
  const struct Model140CallbackAdapter *model_140_callback_adapter;
  const struct Model140StatefulAdapter *model_140_stateful_adapter;
  const struct Model141CallbackAdapter *model_141_callback_adapter;
  const struct Model141StatefulAdapter *model_141_stateful_adapter;
  const struct Model142CallbackAdapter *model_142_callback_adapter;
  const struct Model142StatefulAdapter *model_142_stateful_adapter;
  const struct Model143CallbackAdapter *model_143_callback_adapter;
  const struct Model143StatefulAdapter *model_143_stateful_adapter;
  const struct Model144CallbackAdapter *model_144_callback_adapter;
  const struct Model144StatefulAdapter *model_144_stateful_adapter;
  const struct Model145CallbackAdapter *model_145_callback_adapter;
  const struct Model145StatefulAdapter *model_145_stateful_adapter;
  const struct Model160CallbackAdapter *model_160_callback_adapter;
  const struct Model160StatefulAdapter *model_160_stateful_adapter;
  const struct Model201CallbackAdapter *model_201_callback_adapter;
  const struct Model201StatefulAdapter *model_201_stateful_adapter;
  const struct Model202CallbackAdapter *model_202_callback_adapter;
  const struct Model202StatefulAdapter *model_202_stateful_adapter;
  const struct Model203CallbackAdapter *model_203_callback_adapter;
  const struct Model203StatefulAdapter *model_203_stateful_adapter;
  const struct Model204CallbackAdapter *model_204_callback_adapter;
  const struct Model204StatefulAdapter *model_204_stateful_adapter;
  const struct Model211CallbackAdapter *model_211_callback_adapter;
  const struct Model211StatefulAdapter *model_211_stateful_adapter;
  const struct Model212CallbackAdapter *model_212_callback_adapter;
  const struct Model212StatefulAdapter *model_212_stateful_adapter;
  const struct Model213CallbackAdapter *model_213_callback_adapter;
  const struct Model213StatefulAdapter *model_213_stateful_adapter;
  const struct Model214CallbackAdapter *model_214_callback_adapter;
  const struct Model214StatefulAdapter *model_214_stateful_adapter;
  const struct Model220CallbackAdapter *model_220_callback_adapter;
  const struct Model220StatefulAdapter *model_220_stateful_adapter;
  const struct Model305CallbackAdapter *model_305_callback_adapter;
  const struct Model305StatefulAdapter *model_305_stateful_adapter;
  const struct Model306CallbackAdapter *model_306_callback_adapter;
  const struct Model306StatefulAdapter *model_306_stateful_adapter;
  const struct Model307CallbackAdapter *model_307_callback_adapter;
  const struct Model307StatefulAdapter *model_307_stateful_adapter;
  const struct Model308CallbackAdapter *model_308_callback_adapter;
  const struct Model308StatefulAdapter *model_308_stateful_adapter;
  const struct Model401CallbackAdapter *model_401_callback_adapter;
  const struct Model401StatefulAdapter *model_401_stateful_adapter;
  const struct Model402CallbackAdapter *model_402_callback_adapter;
  const struct Model402StatefulAdapter *model_402_stateful_adapter;
  const struct Model403CallbackAdapter *model_403_callback_adapter;
  const struct Model403StatefulAdapter *model_403_stateful_adapter;
  const struct Model404CallbackAdapter *model_404_callback_adapter;
  const struct Model404StatefulAdapter *model_404_stateful_adapter;
  const struct Model501CallbackAdapter *model_501_callback_adapter;
  const struct Model501StatefulAdapter *model_501_stateful_adapter;
  const struct Model502CallbackAdapter *model_502_callback_adapter;
  const struct Model502StatefulAdapter *model_502_stateful_adapter;
  const struct Model701CallbackAdapter *model_701_callback_adapter;
  const struct Model701StatefulAdapter *model_701_stateful_adapter;
  const struct Model703CallbackAdapter *model_703_callback_adapter;
  const struct Model703StatefulAdapter *model_703_stateful_adapter;
  const struct Model704CallbackAdapter *model_704_callback_adapter;
  const struct Model704StatefulAdapter *model_704_stateful_adapter;
  const struct Model705CallbackAdapter *model_705_callback_adapter;
  const struct Model705StatefulAdapter *model_705_stateful_adapter;
  const struct Model706CallbackAdapter *model_706_callback_adapter;
  const struct Model706StatefulAdapter *model_706_stateful_adapter;
  const struct Model707CallbackAdapter *model_707_callback_adapter;
  const struct Model707StatefulAdapter *model_707_stateful_adapter;
  const struct Model708CallbackAdapter *model_708_callback_adapter;
  const struct Model708StatefulAdapter *model_708_stateful_adapter;
  const struct Model709CallbackAdapter *model_709_callback_adapter;
  const struct Model709StatefulAdapter *model_709_stateful_adapter;
  const struct Model710CallbackAdapter *model_710_callback_adapter;
  const struct Model710StatefulAdapter *model_710_stateful_adapter;
  const struct Model711CallbackAdapter *model_711_callback_adapter;
  const struct Model711StatefulAdapter *model_711_stateful_adapter;
  const struct Model712CallbackAdapter *model_712_callback_adapter;
  const struct Model712StatefulAdapter *model_712_stateful_adapter;
  const struct Model713CallbackAdapter *model_713_callback_adapter;
  const struct Model713StatefulAdapter *model_713_stateful_adapter;
  const struct Model714CallbackAdapter *model_714_callback_adapter;
  const struct Model714StatefulAdapter *model_714_stateful_adapter;
  const struct Model715CallbackAdapter *model_715_callback_adapter;
  const struct Model715StatefulAdapter *model_715_stateful_adapter;
  const struct Model801CallbackAdapter *model_801_callback_adapter;
  const struct Model801StatefulAdapter *model_801_stateful_adapter;
  const struct Model802CallbackAdapter *model_802_callback_adapter;
  const struct Model802StatefulAdapter *model_802_stateful_adapter;
  const struct Model803CallbackAdapter *model_803_callback_adapter;
  const struct Model803StatefulAdapter *model_803_stateful_adapter;
  const struct Model804CallbackAdapter *model_804_callback_adapter;
  const struct Model804StatefulAdapter *model_804_stateful_adapter;
  const struct Model805CallbackAdapter *model_805_callback_adapter;
  const struct Model805StatefulAdapter *model_805_stateful_adapter;
  const struct Model806CallbackAdapter *model_806_callback_adapter;
  const struct Model806StatefulAdapter *model_806_stateful_adapter;
  const struct Model807CallbackAdapter *model_807_callback_adapter;
  const struct Model807StatefulAdapter *model_807_stateful_adapter;
  const struct Model808CallbackAdapter *model_808_callback_adapter;
  const struct Model808StatefulAdapter *model_808_stateful_adapter;
  const struct Model809CallbackAdapter *model_809_callback_adapter;
  const struct Model809StatefulAdapter *model_809_stateful_adapter;
  const struct Model63001CallbackAdapter *model_63001_callback_adapter;
  const struct Model63001StatefulAdapter *model_63001_stateful_adapter;
  const struct Model64001CallbackAdapter *model_64001_callback_adapter;
  const struct Model64001StatefulAdapter *model_64001_stateful_adapter;
  const struct Model64020CallbackAdapter *model_64020_callback_adapter;
  const struct Model64020StatefulAdapter *model_64020_stateful_adapter;
  const struct Model64101CallbackAdapter *model_64101_callback_adapter;
  const struct Model64101StatefulAdapter *model_64101_stateful_adapter;
  const struct Model64111CallbackAdapter *model_64111_callback_adapter;
  const struct Model64111StatefulAdapter *model_64111_stateful_adapter;
  const struct Model64112CallbackAdapter *model_64112_callback_adapter;
  const struct Model64112StatefulAdapter *model_64112_stateful_adapter;
  const struct Model64410CallbackAdapter *model_64410_callback_adapter;
  const struct Model64410StatefulAdapter *model_64410_stateful_adapter;
  const struct Model64411CallbackAdapter *model_64411_callback_adapter;
  const struct Model64411StatefulAdapter *model_64411_stateful_adapter;
  const struct Model64412CallbackAdapter *model_64412_callback_adapter;
  const struct Model64412StatefulAdapter *model_64412_stateful_adapter;
  const struct Model64413CallbackAdapter *model_64413_callback_adapter;
  const struct Model64413StatefulAdapter *model_64413_stateful_adapter;
  const struct Model64414CallbackAdapter *model_64414_callback_adapter;
  const struct Model64414StatefulAdapter *model_64414_stateful_adapter;
  const struct Model64415CallbackAdapter *model_64415_callback_adapter;
  const struct Model64415StatefulAdapter *model_64415_stateful_adapter;
} SunspecExternalAdapters;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

extern void handle_panic(const char *message);

extern int32_t printf(const char *format, ...);

int32_t sunspec_service_handle_request(const struct SunspecExternalAdapters *adapters,
                                       uint16_t address,
                                       uint16_t length,
                                       uint8_t *response_buffer);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  /* SUNSPEC_MODBUS_CODEC_H */
