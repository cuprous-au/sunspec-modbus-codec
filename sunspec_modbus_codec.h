#ifndef SUNSPEC_MODBUS_CODEC_H
#define SUNSPEC_MODBUS_CODEC_H

#pragma once

/* Generated with cbindgen:0.29.4 */

enum St
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  Down = 0,
  Up = 1,
  Fault = 2,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum St St;
#else
typedef uint16_t St;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum Typ
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  Unknown = 0,
  Internal = 1,
  TwistedPair = 2,
  Fiber = 3,
  Wireless = 4,
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
  NotConfigured = 0,
  ValidSetting = 1,
  ValidHw = 2,
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
  Static = 0,
  Dhcp = 1,
  Bootp = 2,
  Zeroconf = 3,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum Cfg Cfg;
#else
typedef uint16_t Cfg;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum Ctl
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  EnableDns = 0,
  EnableNtp = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum Ctl Ctl;
#else
typedef uint16_t Ctl;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum DerTyp
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  Pv = 4,
  PvStor = 82,
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
  Switch = 1,
  Maintain = 2,
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
  Vector = 1,
  Arithmetic = 2,
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
  A = 1,
  B = 2,
  C = 3,
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
  Disconnect = 0,
  Connect = 1,
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
  Disabled = 0,
  Enabled = 1,
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
  Disabled = 0,
  Enabled = 1,
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
  None = 0,
  WMax = 1,
  VArMax = 2,
  VArAval = 3,
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
  Disabled = 0,
  Enabled = 1,
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
  Off = 1,
  Empty = 2,
  Discharging = 3,
  Charging = 4,
  Full = 5,
  Holding = 6,
  Testing = 7,
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
  Pv = 0,
  Grid = 1,
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
  Unknown = 0,
  Absolute = 1,
  Relative = 2,
  Multiplier = 3,
  Level = 4,
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
  Edge = 0,
  Center = 1,
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
  CeaseToEnergize = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum CrvType CrvType;
#else
typedef uint16_t CrvType;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum Pty
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  None = 0,
  Odd = 1,
  Even = 2,
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
  Full = 0,
  Half = 1,
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
  None = 0,
  Hw = 1,
  Xonxoff = 2,
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
  Unknown = 0,
  Modbus = 1,
  Vendor = 2,
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
  None = 0,
  Pap = 1,
  Chap = 2,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum Auth Auth;
#else
typedef uint16_t Auth;
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
  None = 0,
  AesGmac64 = 1,
  Ecc256 = 2,
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
  Success = 0,
  Ds = 1,
  /*
   One or more registers were not writable by this role
   */
  Acl = 2,
  /*
   Offset out of range or missing from multi-register value
   */
  Off = 3,
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
  None = 0,
  /*
   Tampered
   */
  Alm = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum Alm Alm;
#else
typedef uint16_t Alm;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum Stat
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  Off = 1,
  Sleeping = 2,
  Starting = 3,
  Mppt = 4,
  Throttled = 5,
  ShuttingDown = 6,
  Fault = 7,
  Standby = 8,
  Test = 9,
  Other = 10,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum Stat Stat;
#else
typedef uint16_t Stat;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum ChargerSt
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  Off = 0,
  Float = 1,
  Bulk = 2,
  Absorb = 3,
  Eq = 4,
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
  Auto = 0,
  UPick = 1,
  Wind = 2,
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
  Half = 0,
  Full = 1,
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
  EightyPercent = 0,
  EightyFivePercent = 1,
  NintyPercent = 2,
  NintyNinePercent = 3,
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
  Disabled = 0,
  Enabled = 1,
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
  Wide = 0,
  Limited = 1,
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
  Off = 0,
  Every90Minutes = 1,
  Every90MinutesIfAbsorbOrFloat = 2,
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
  Float = 0,
  DiversionRelay = 1,
  DiversionSolidSt = 2,
  LowBattDisconnect = 3,
  Remote = 4,
  VentFan = 5,
  PvTrigger = 6,
  ErrorOutput = 7,
  NightLight = 8,
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
  Off = 0,
  Auto = 1,
  On = 2,
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
  Disabled = 0,
  Enabled = 1,
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
  Low = 0,
  High = 1,
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
  Cv = 0,
  /*
   CC Mode

   Constant Current (CC) Mode.
   */
  Cc = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum Mode Mode;
#else
typedef uint16_t Mode;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum Ena
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  /*
   Power On

   Power On
   */
  On = 1,
  /*
   Power Off

   Power Off
   */
  Off = 0,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum Ena Ena;
#else
typedef uint16_t Ena;
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
  Reset = 1,
  /*
   Do Not Reset Device

   Do Not Reset Device
   */
  DoNotReset = 0,
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
  En50530 = 1,
  /*
   Do Not Use EN50530 Mode

   Do Not Use EN50530 Mode
   */
  DoNotEn50530 = 0,
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
  Start = 1,
  /*
   Stop Profile

   Stop the Profile
   */
  Stop = 0,
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
  InProgress = 0,
  /*
   Update Complete

   Profile update completed successfully.
   */
  Completed = 1,
  /*
   Update Failed

   Profile update failed.
   */
  Failed = 2,
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
  Off = 0,
  /*
   Output On
   */
  On = 1,
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
  Open = 0,
  /*
   Relay Closed
   */
  Closed = 1,
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
  Off = 0,
  /*
   Regen On
   */
  On = 1,
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
  InProgress = 0,
  /*
   Profile update completed successfully.
   */
  Completed = 1,
  /*
   Profile update failed.
   */
  Failed = 2,
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
  Off = 0,
  /*
   Data Falsification

   Modbus Falsification Enabled
   */
  On = 1,
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
  Off = 0,
  /*
   Data Falsification

   Modbus Falsification Enabled
   */
  On = 1,
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
  Off = 0,
  /*
   Data Falsification

   Modbus Falsification Enabled
   */
  On = 1,
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
  Off = 0,
  /*
   Data Falsification

   Modbus Falsification Enabled
   */
  On = 1,
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
  Off = 0,
  /*
   Data Falsification

   Modbus Falsification Enabled
   */
  On = 1,
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
  Off = 0,
  /*
   Data Falsification

   Modbus Falsification Enabled
   */
  On = 1,
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
  Off = 0,
  /*
   Data Falsification

   Modbus Falsification Enabled
   */
  On = 1,
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
  Off = 0,
  /*
   Data Falsification

   Modbus Falsification Enabled
   */
  On = 1,
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
  Off = 0,
  /*
   Data Falsification

   Modbus Falsification Enabled
   */
  On = 1,
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
  Off = 0,
  /*
   Data Falsification

   Modbus Falsification Enabled
   */
  On = 1,
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
  Off = 0,
  /*
   Data Falsification

   Modbus Falsification Enabled
   */
  On = 1,
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
  Off = 0,
  /*
   Data Falsification

   Modbus Falsification Enabled
   */
  On = 1,
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
  Off = 0,
  /*
   Data Falsification

   Modbus Falsification Enabled
   */
  On = 1,
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
  Off = 0,
  /*
   Data Falsification

   Modbus Falsification Enabled
   */
  On = 1,
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
  Off = 0,
  /*
   Data Falsification

   Modbus Falsification Enabled
   */
  On = 1,
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
  Off = 0,
  /*
   Data Falsification

   Modbus Falsification Enabled
   */
  On = 1,
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
  Off = 0,
  /*
   Data Falsification

   Modbus Falsification Enabled
   */
  On = 1,
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
  Off = 0,
  /*
   Data Falsification

   Modbus Falsification Enabled
   */
  On = 1,
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
  Off = 0,
  /*
   Data Falsification

   Modbus Falsification Enabled
   */
  On = 1,
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
  Off = 0,
  /*
   Data Falsification

   Modbus Falsification Enabled
   */
  On = 1,
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
  Off = 0,
  /*
   Data Falsification

   Modbus Falsification Enabled
   */
  On = 1,
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
  Off = 0,
  /*
   Data Falsification

   Modbus Falsification Enabled
   */
  On = 1,
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
  Off = 0,
  /*
   Data Falsification

   Modbus Falsification Enabled
   */
  On = 1,
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
  Off = 0,
  /*
   Data Falsification

   Modbus Falsification Enabled
   */
  On = 1,
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
  Off = 0,
  /*
   Data Falsification

   Modbus Falsification Enabled
   */
  On = 1,
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
  Off = 0,
  /*
   Data Falsification

   Modbus Falsification Enabled
   */
  On = 1,
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
  Off = 0,
  /*
   Data Falsification

   Modbus Falsification Enabled
   */
  On = 1,
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
  Off = 0,
  /*
   Data Falsification

   Modbus Falsification Enabled
   */
  On = 1,
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
  Off = 0,
  /*
   Data Falsification

   Modbus Falsification Enabled
   */
  On = 1,
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
  Off = 0,
  /*
   Data Falsification

   Modbus Falsification Enabled
   */
  On = 1,
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
  Off = 0,
  /*
   Data Falsification

   Modbus Falsification Enabled
   */
  On = 1,
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
  Off = 0,
  /*
   Data Falsification

   Modbus Falsification Enabled
   */
  On = 1,
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
  Off = 0,
  /*
   Data Falsification

   Modbus Falsification Enabled
   */
  On = 1,
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
  Off = 0,
  /*
   Data Falsification

   Modbus Falsification Enabled
   */
  On = 1,
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
  Off = 0,
  /*
   Data Falsification

   Modbus Falsification Enabled
   */
  On = 1,
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
  Off = 0,
  /*
   Data Falsification

   Modbus Falsification Enabled
   */
  On = 1,
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
  Off = 0,
  /*
   Data Falsification

   Modbus Falsification Enabled
   */
  On = 1,
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
  Off = 0,
  /*
   Data Falsification

   Modbus Falsification Enabled
   */
  On = 1,
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
  Off = 0,
  /*
   Data Falsification

   Modbus Falsification Enabled
   */
  On = 1,
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
  Off = 0,
  /*
   Data Falsification

   Modbus Falsification Enabled
   */
  On = 1,
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
  Off = 0,
  /*
   Data Falsification

   Modbus Falsification Enabled
   */
  On = 1,
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
  Off = 0,
  /*
   Data Falsification

   Modbus Falsification Enabled
   */
  On = 1,
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
  Off = 0,
  /*
   Data Falsification

   Modbus Falsification Enabled
   */
  On = 1,
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
  Disabled = 0,
  /*
   Enabled

   LogEvent Mode Enabled
   */
  Enabled = 1,
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
  Disabled = 0,
  /*
   Enabled

   HTTP Message Mode Enabled
   */
  Enabled = 1,
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
  DefaultCertificate = 0,
  /*
   COMM-004A

   Chain Length Two Certificate
   */
  Comm004a = 1,
  /*
   COMM-004B

   Chain Length Three Certificate
   */
  Comm004b = 2,
  /*
   COMM-004C

   Chain Length Four Certificate
   */
  Comm004c = 3,
  /*
   COMM-004D

   Invalid MICA Extended Key Critical Value
   */
  Comm004d = 4,
  /*
   COMM-004E

   Invalid MICA Name Non-Critical Value
   */
  Comm004e = 5,
  /*
   COMM-004F

   Invalid MICA Policy Mapping Non-Critical Value
   */
  Comm004f = 6,
  /*
   COMM-004G

   Self-signed device certificate
   */
  Comm004g = 7,
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
  Disabled = 0,
  /*
   Enabled

   Subscription Mode Enabled
   */
  Enabled = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum SubscriptionEna SubscriptionEna;
#else
typedef uint16_t SubscriptionEna;
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
  SinglePhase = 0,
  /*
   Split Phase
   */
  SplitPhase = 1,
  /*
   Three Phase
   */
  ThreePhase = 2,
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
  Off = 0,
  Sleeping = 1,
  Starting = 2,
  Running = 3,
  Throttled = 4,
  ShuttingDown = 5,
  Fault = 6,
  Standby = 7,
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
  Disconnected = 0,
  /*
   Connected

   Connected to the grid.
   */
  Connected = 1,
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
  Disabled = 0,
  Enabled = 1,
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
  Disabled = 0,
  /*
   Enabled

   Function is enabled.
   */
  Enabled = 1,
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
  Disabled = 0,
  /*
   Enabled

   Function is enabled.
   */
  Enabled = 1,
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
  Disabled = 0,
  /*
   Enabled

   Function is enabled.
   */
  Enabled = 1,
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
  Disabled = 0,
  /*
   Enabled

   Function is enabled.
   */
  Enabled = 1,
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
  Disabled = 0,
  /*
   Enabled

   Function is enabled.
   */
  Enabled = 1,
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
  Disabled = 0,
  /*
   Enabled

   Function is enabled.
   */
  Enabled = 1,
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
  Disabled = 0,
  /*
   Enabled

   Function is enabled.
   */
  Enabled = 1,
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
  WMaxPct = 0,
  /*
   Active Power As Watts

   Active power setting is in watts.
   */
  Watts = 1,
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
  Disabled = 0,
  /*
   Enabled

   Function is enabled.
   */
  Enabled = 1,
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
  Disabled = 0,
  /*
   Enabled

   Function is enabled.
   */
  Enabled = 1,
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
  WMaxPct = 0,
  /*
   Reactive Power As Var Max Pct

   Reactive power setting is percent of maximum reactive power.
   */
  VarMaxPct = 1,
  /*
   Reactive Power As Var Avail Pct

   Reactive power setting is percent of available reactive  power.
   */
  VarAvailPct = 2,
  /*
   Reactive Power As VA Max Pct

   Reactive power setting is percent of maximum apparent power.
   */
  VaMaxPct = 3,
  /*
   Reactive Power As Vars

   Reactive power is in vars.
   */
  Vars = 4,
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
  Active = 0,
  /*
   Reactive Power Priority

   Reactive power priority.
   */
  Reactive = 1,
  /*
   Vendor Power Priority

   Power priority is vendor specific mode.
   */
  Vendor = 2,
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
  Disabled = 0,
  /*
   Enabled

   Function is enabled.
   */
  Enabled = 1,
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
  AMax = 0,
  /*
   Max Active Power Ramp

   Ramp based on percent of max active power per second.
   */
  WMax = 1,
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
  Disabled = 0,
  /*
   Enabled

   Anti-islanding is enabled.
   */
  Enabled = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum AntiIslEna AntiIslEna;
#else
typedef uint16_t AntiIslEna;
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
  InProgress = 0,
  /*
   Update Complete

   Curve update completed successfully.
   */
  Completed = 1,
  /*
   Update Failed

   Curve update failed.
   */
  Failed = 2,
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
  InProgress = 0,
  /*
   Update Complete

   Control update completed successfully.
   */
  Completed = 1,
  /*
   Update Failed

   Control update failed.
   */
  Failed = 2,
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
  Ok = 0,
  /*
   Warning

   One or more warnings pending.
   */
  Warning = 1,
  /*
   Error

   One or more errors pending.
   */
  Error = 2,
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
  Remote = 0,
  /*
   Local Control

   Local mode is required for manual/maintenance operations. Once invoked, it must be explicitly exited for the inverter to be controlled remotely.
   */
  Local = 1,
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
  Stop = 0,
  /*
   Start the DER
   */
  Start = 1,
  /*
   Enter Standby Mode
   */
  EnterStandby = 2,
  /*
   Exit Standby Mode
   */
  ExitStandby = 3,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum OpCtl OpCtl;
#else
typedef uint16_t OpCtl;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum Fmt
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  None = 0,
  X509Pem = 1,
  X509Der = 2,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum Fmt Fmt;
#else
typedef uint16_t Fmt;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum State
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint16_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  Disconnected = 1,
  Initializing = 2,
  Connected = 3,
  Standby = 4,
  SocProtection = 5,
  Suspending = 6,
  Fault = 99,
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
  NoRequest = 0,
  /*
   Battery is notified of inverter state change through SetInvState.
   */
  Start = 1,
  /*
   Battery is notified of inverter state change through SetInvState.
   */
  Stop = 2,
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
  Connect = 1,
  Disconnect = 2,
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
  InverterStopped = 1,
  InverterStandby = 2,
  InverterStarted = 3,
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
  NoFailure = 0,
  ButtonPushed = 1,
  StrGroundFault = 2,
  OutsideVoltageRange = 3,
  StringNotEnabled = 4,
  FuseOpen = 5,
  ContactorFailure = 6,
  PrechargeFailure = 7,
  /*
   See Evt1 for more information.
   */
  StringFault = 8,
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
  ConnectString = 1,
  DisconnectString = 2,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum SetCon SetCon;
#else
typedef uint16_t SetCon;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

typedef struct Model1CallbackAdapter {
  const char *(*manufacturer_callback)(void);
  const char *(*model_callback)(void);
  const char *(*options_callback)(void);
  const char *(*version_callback)(void);
  const char *(*serial_number_callback)(void);
  uint16_t (*device_address_callback)(void);
  void (*set_device_address_callback)(uint16_t);
} Model1CallbackAdapter;

typedef struct Model10CallbackAdapter {
  St (*interface_status_callback)(void);
  uint16_t (*interface_control_callback)(void);
  void (*set_interface_control_callback)(uint16_t);
  Typ (*physical_access_type_callback)(void);
} Model10CallbackAdapter;

typedef struct Model101CallbackAdapter {
  uint16_t (*amps_callback)(void);
  uint16_t (*amps_phase_a_callback)(void);
  uint16_t (*amps_phase_b_callback)(void);
  uint16_t (*amps_phase_c_callback)(void);
  uint16_t (*a_sf_callback)(void);
  uint16_t (*phase_voltage_ab_callback)(void);
  uint16_t (*phase_voltage_bc_callback)(void);
  uint16_t (*phase_voltage_ca_callback)(void);
  uint16_t (*phase_voltage_an_callback)(void);
  uint16_t (*phase_voltage_bn_callback)(void);
  uint16_t (*phase_voltage_cn_callback)(void);
  uint16_t (*v_sf_callback)(void);
  int16_t (*watts_callback)(void);
  uint16_t (*w_sf_callback)(void);
  uint16_t (*hz_callback)(void);
  uint16_t (*hz_sf_callback)(void);
  int16_t (*va_callback)(void);
  uint16_t (*va_sf_callback)(void);
  int16_t (*v_ar_callback)(void);
  uint16_t (*v_ar_sf_callback)(void);
  int16_t (*pf_callback)(void);
  uint16_t (*pf_sf_callback)(void);
  uint32_t (*watt_hours_callback)(void);
  uint16_t (*wh_sf_callback)(void);
  uint16_t (*dc_amps_callback)(void);
  uint16_t (*dca_sf_callback)(void);
  uint16_t (*dc_voltage_callback)(void);
  uint16_t (*dcv_sf_callback)(void);
  int16_t (*dc_watts_callback)(void);
  uint16_t (*dcw_sf_callback)(void);
  int16_t (*cabinet_temperature_callback)(void);
  int16_t (*heat_sink_temperature_callback)(void);
  int16_t (*transformer_temperature_callback)(void);
  int16_t (*other_temperature_callback)(void);
  uint16_t (*tmp_sf_callback)(void);
  St (*operating_state_callback)(void);
  uint16_t (*vendor_operating_state_callback)(void);
  uint32_t (*event1_callback)(void);
  uint32_t (*event_bitfield_2_callback)(void);
  uint32_t (*vendor_event_bitfield_1_callback)(void);
  uint32_t (*vendor_event_bitfield_2_callback)(void);
  uint32_t (*vendor_event_bitfield_3_callback)(void);
  uint32_t (*vendor_event_bitfield_4_callback)(void);
} Model101CallbackAdapter;

typedef struct Model102CallbackAdapter {
  uint16_t (*amps_callback)(void);
  uint16_t (*amps_phase_a_callback)(void);
  uint16_t (*amps_phase_b_callback)(void);
  uint16_t (*amps_phase_c_callback)(void);
  uint16_t (*a_sf_callback)(void);
  uint16_t (*phase_voltage_ab_callback)(void);
  uint16_t (*phase_voltage_bc_callback)(void);
  uint16_t (*phase_voltage_ca_callback)(void);
  uint16_t (*phase_voltage_an_callback)(void);
  uint16_t (*phase_voltage_bn_callback)(void);
  uint16_t (*phase_voltage_cn_callback)(void);
  uint16_t (*v_sf_callback)(void);
  int16_t (*watts_callback)(void);
  uint16_t (*w_sf_callback)(void);
  uint16_t (*hz_callback)(void);
  uint16_t (*hz_sf_callback)(void);
  int16_t (*va_callback)(void);
  uint16_t (*va_sf_callback)(void);
  int16_t (*v_ar_callback)(void);
  uint16_t (*v_ar_sf_callback)(void);
  int16_t (*pf_callback)(void);
  uint16_t (*pf_sf_callback)(void);
  uint32_t (*watt_hours_callback)(void);
  uint16_t (*wh_sf_callback)(void);
  uint16_t (*dc_amps_callback)(void);
  uint16_t (*dca_sf_callback)(void);
  uint16_t (*dc_voltage_callback)(void);
  uint16_t (*dcv_sf_callback)(void);
  int16_t (*dc_watts_callback)(void);
  uint16_t (*dcw_sf_callback)(void);
  int16_t (*cabinet_temperature_callback)(void);
  int16_t (*heat_sink_temperature_callback)(void);
  int16_t (*transformer_temperature_callback)(void);
  int16_t (*other_temperature_callback)(void);
  uint16_t (*tmp_sf_callback)(void);
  St (*operating_state_callback)(void);
  uint16_t (*vendor_operating_state_callback)(void);
  uint32_t (*event1_callback)(void);
  uint32_t (*event_bitfield_2_callback)(void);
  uint32_t (*vendor_event_bitfield_1_callback)(void);
  uint32_t (*vendor_event_bitfield_2_callback)(void);
  uint32_t (*vendor_event_bitfield_3_callback)(void);
  uint32_t (*vendor_event_bitfield_4_callback)(void);
} Model102CallbackAdapter;

typedef struct Model103CallbackAdapter {
  uint16_t (*amps_callback)(void);
  uint16_t (*amps_phase_a_callback)(void);
  uint16_t (*amps_phase_b_callback)(void);
  uint16_t (*amps_phase_c_callback)(void);
  uint16_t (*a_sf_callback)(void);
  uint16_t (*phase_voltage_ab_callback)(void);
  uint16_t (*phase_voltage_bc_callback)(void);
  uint16_t (*phase_voltage_ca_callback)(void);
  uint16_t (*phase_voltage_an_callback)(void);
  uint16_t (*phase_voltage_bn_callback)(void);
  uint16_t (*phase_voltage_cn_callback)(void);
  uint16_t (*v_sf_callback)(void);
  int16_t (*watts_callback)(void);
  uint16_t (*w_sf_callback)(void);
  uint16_t (*hz_callback)(void);
  uint16_t (*hz_sf_callback)(void);
  int16_t (*va_callback)(void);
  uint16_t (*va_sf_callback)(void);
  int16_t (*v_ar_callback)(void);
  uint16_t (*v_ar_sf_callback)(void);
  int16_t (*pf_callback)(void);
  uint16_t (*pf_sf_callback)(void);
  uint32_t (*watt_hours_callback)(void);
  uint16_t (*wh_sf_callback)(void);
  uint16_t (*dc_amps_callback)(void);
  uint16_t (*dca_sf_callback)(void);
  uint16_t (*dc_voltage_callback)(void);
  uint16_t (*dcv_sf_callback)(void);
  int16_t (*dc_watts_callback)(void);
  uint16_t (*dcw_sf_callback)(void);
  int16_t (*cabinet_temperature_callback)(void);
  int16_t (*heat_sink_temperature_callback)(void);
  int16_t (*transformer_temperature_callback)(void);
  int16_t (*other_temperature_callback)(void);
  uint16_t (*tmp_sf_callback)(void);
  St (*operating_state_callback)(void);
  uint16_t (*vendor_operating_state_callback)(void);
  uint32_t (*event1_callback)(void);
  uint32_t (*event_bitfield_2_callback)(void);
  uint32_t (*vendor_event_bitfield_1_callback)(void);
  uint32_t (*vendor_event_bitfield_2_callback)(void);
  uint32_t (*vendor_event_bitfield_3_callback)(void);
  uint32_t (*vendor_event_bitfield_4_callback)(void);
} Model103CallbackAdapter;

typedef struct Model11CallbackAdapter {
  uint16_t (*ethernet_link_speed_callback)(void);
  uint16_t (*interface_status_flags_callback)(void);
  St (*link_state_callback)(void);
  const uint8_t *(*mac_callback)(void);
  const char *(*name_callback)(void);
  void (*set_name_callback)(const char*);
  uint16_t (*control_callback)(void);
  void (*set_control_callback)(uint16_t);
  uint16_t (*forced_speed_callback)(void);
  void (*set_forced_speed_callback)(uint16_t);
} Model11CallbackAdapter;

typedef struct Model111CallbackAdapter {
  float (*amps_callback)(void);
  float (*amps_phase_a_callback)(void);
  float (*amps_phase_b_callback)(void);
  float (*amps_phase_c_callback)(void);
  float (*phase_voltage_ab_callback)(void);
  float (*phase_voltage_bc_callback)(void);
  float (*phase_voltage_ca_callback)(void);
  float (*phase_voltage_an_callback)(void);
  float (*phase_voltage_bn_callback)(void);
  float (*phase_voltage_cn_callback)(void);
  float (*watts_callback)(void);
  float (*hz_callback)(void);
  float (*va_callback)(void);
  float (*v_ar_callback)(void);
  float (*pf_callback)(void);
  float (*watt_hours_callback)(void);
  float (*dc_amps_callback)(void);
  float (*dc_voltage_callback)(void);
  float (*dc_watts_callback)(void);
  float (*cabinet_temperature_callback)(void);
  float (*heat_sink_temperature_callback)(void);
  float (*transformer_temperature_callback)(void);
  float (*other_temperature_callback)(void);
  St (*operating_state_callback)(void);
  uint16_t (*vendor_operating_state_callback)(void);
  uint32_t (*event1_callback)(void);
  uint32_t (*event_bitfield_2_callback)(void);
  uint32_t (*vendor_event_bitfield_1_callback)(void);
  uint32_t (*vendor_event_bitfield_2_callback)(void);
  uint32_t (*vendor_event_bitfield_3_callback)(void);
  uint32_t (*vendor_event_bitfield_4_callback)(void);
} Model111CallbackAdapter;

typedef struct Model112CallbackAdapter {
  float (*amps_callback)(void);
  float (*amps_phase_a_callback)(void);
  float (*amps_phase_b_callback)(void);
  float (*amps_phase_c_callback)(void);
  float (*phase_voltage_ab_callback)(void);
  float (*phase_voltage_bc_callback)(void);
  float (*phase_voltage_ca_callback)(void);
  float (*phase_voltage_an_callback)(void);
  float (*phase_voltage_bn_callback)(void);
  float (*phase_voltage_cn_callback)(void);
  float (*watts_callback)(void);
  float (*hz_callback)(void);
  float (*va_callback)(void);
  float (*v_ar_callback)(void);
  float (*pf_callback)(void);
  float (*watt_hours_callback)(void);
  float (*dc_amps_callback)(void);
  float (*dc_voltage_callback)(void);
  float (*dc_watts_callback)(void);
  float (*cabinet_temperature_callback)(void);
  float (*heat_sink_temperature_callback)(void);
  float (*transformer_temperature_callback)(void);
  float (*other_temperature_callback)(void);
  St (*operating_state_callback)(void);
  uint16_t (*vendor_operating_state_callback)(void);
  uint32_t (*event1_callback)(void);
  uint32_t (*event_bitfield_2_callback)(void);
  uint32_t (*vendor_event_bitfield_1_callback)(void);
  uint32_t (*vendor_event_bitfield_2_callback)(void);
  uint32_t (*vendor_event_bitfield_3_callback)(void);
  uint32_t (*vendor_event_bitfield_4_callback)(void);
} Model112CallbackAdapter;

typedef struct Model113CallbackAdapter {
  float (*amps_callback)(void);
  float (*amps_phase_a_callback)(void);
  float (*amps_phase_b_callback)(void);
  float (*amps_phase_c_callback)(void);
  float (*phase_voltage_ab_callback)(void);
  float (*phase_voltage_bc_callback)(void);
  float (*phase_voltage_ca_callback)(void);
  float (*phase_voltage_an_callback)(void);
  float (*phase_voltage_bn_callback)(void);
  float (*phase_voltage_cn_callback)(void);
  float (*watts_callback)(void);
  float (*hz_callback)(void);
  float (*va_callback)(void);
  float (*v_ar_callback)(void);
  float (*pf_callback)(void);
  float (*watt_hours_callback)(void);
  float (*dc_amps_callback)(void);
  float (*dc_voltage_callback)(void);
  float (*dc_watts_callback)(void);
  float (*cabinet_temperature_callback)(void);
  float (*heat_sink_temperature_callback)(void);
  float (*transformer_temperature_callback)(void);
  float (*other_temperature_callback)(void);
  St (*operating_state_callback)(void);
  uint16_t (*vendor_operating_state_callback)(void);
  uint32_t (*event1_callback)(void);
  uint32_t (*event_bitfield_2_callback)(void);
  uint32_t (*vendor_event_bitfield_1_callback)(void);
  uint32_t (*vendor_event_bitfield_2_callback)(void);
  uint32_t (*vendor_event_bitfield_3_callback)(void);
  uint32_t (*vendor_event_bitfield_4_callback)(void);
} Model113CallbackAdapter;

typedef struct Model12CallbackAdapter {
  const char *(*name_callback)(void);
  void (*set_name_callback)(const char*);
  CfgSt (*config_status_callback)(void);
  uint16_t (*change_status_callback)(void);
  uint16_t (*config_capability_callback)(void);
  Cfg (*i_pv4_config_callback)(void);
  void (*set_i_pv4_config_callback)(Cfg);
  Ctl (*control_callback)(void);
  void (*set_control_callback)(Ctl);
  const char *(*ip_callback)(void);
  void (*set_ip_callback)(const char*);
  const char *(*netmask_callback)(void);
  void (*set_netmask_callback)(const char*);
  const char *(*gateway_callback)(void);
  void (*set_gateway_callback)(const char*);
  const char *(*dns1_callback)(void);
  void (*set_dns1_callback)(const char*);
  const char *(*dns2_callback)(void);
  void (*set_dns2_callback)(const char*);
  const char *(*ntp1_callback)(void);
  void (*set_ntp1_callback)(const char*);
  const char *(*ntp2_callback)(void);
  void (*set_ntp2_callback)(const char*);
  const char *(*domain_callback)(void);
  void (*set_domain_callback)(const char*);
  const char *(*host_name_callback)(void);
  void (*set_host_name_callback)(const char*);
} Model12CallbackAdapter;

typedef struct Model120CallbackAdapter {
  DerTyp (*der_typ_callback)(void);
  uint16_t (*w_rtg_callback)(void);
  uint16_t (*w_rtg_sf_callback)(void);
  uint16_t (*va_rtg_callback)(void);
  uint16_t (*va_rtg_sf_callback)(void);
  int16_t (*v_ar_rtg_q1_callback)(void);
  int16_t (*v_ar_rtg_q2_callback)(void);
  int16_t (*v_ar_rtg_q3_callback)(void);
  int16_t (*v_ar_rtg_q4_callback)(void);
  uint16_t (*v_ar_rtg_sf_callback)(void);
  uint16_t (*a_rtg_callback)(void);
  uint16_t (*a_rtg_sf_callback)(void);
  int16_t (*pf_rtg_q1_callback)(void);
  int16_t (*pf_rtg_q2_callback)(void);
  int16_t (*pf_rtg_q3_callback)(void);
  int16_t (*pf_rtg_q4_callback)(void);
  uint16_t (*pf_rtg_sf_callback)(void);
  uint16_t (*wh_rtg_callback)(void);
  uint16_t (*wh_rtg_sf_callback)(void);
  uint16_t (*ahr_rtg_callback)(void);
  uint16_t (*ahr_rtg_sf_callback)(void);
  uint16_t (*max_cha_rte_callback)(void);
  uint16_t (*max_cha_rte_sf_callback)(void);
  uint16_t (*max_dis_cha_rte_callback)(void);
  uint16_t (*max_dis_cha_rte_sf_callback)(void);
} Model120CallbackAdapter;

typedef struct Model121CallbackAdapter {
  uint16_t (*w_max_callback)(void);
  void (*set_w_max_callback)(uint16_t);
  uint16_t (*v_ref_callback)(void);
  void (*set_v_ref_callback)(uint16_t);
  int16_t (*v_ref_ofs_callback)(void);
  void (*set_v_ref_ofs_callback)(int16_t);
  uint16_t (*v_max_callback)(void);
  void (*set_v_max_callback)(uint16_t);
  uint16_t (*v_min_callback)(void);
  void (*set_v_min_callback)(uint16_t);
  uint16_t (*va_max_callback)(void);
  void (*set_va_max_callback)(uint16_t);
  int16_t (*v_ar_max_q1_callback)(void);
  void (*set_v_ar_max_q1_callback)(int16_t);
  int16_t (*v_ar_max_q2_callback)(void);
  void (*set_v_ar_max_q2_callback)(int16_t);
  int16_t (*v_ar_max_q3_callback)(void);
  void (*set_v_ar_max_q3_callback)(int16_t);
  int16_t (*v_ar_max_q4_callback)(void);
  void (*set_v_ar_max_q4_callback)(int16_t);
  uint16_t (*w_gra_callback)(void);
  void (*set_w_gra_callback)(uint16_t);
  int16_t (*pf_min_q1_callback)(void);
  void (*set_pf_min_q1_callback)(int16_t);
  int16_t (*pf_min_q2_callback)(void);
  void (*set_pf_min_q2_callback)(int16_t);
  int16_t (*pf_min_q3_callback)(void);
  void (*set_pf_min_q3_callback)(int16_t);
  int16_t (*pf_min_q4_callback)(void);
  void (*set_pf_min_q4_callback)(int16_t);
  VArAct (*v_ar_act_callback)(void);
  void (*set_v_ar_act_callback)(VArAct);
  ClcTotVa (*clc_tot_va_callback)(void);
  void (*set_clc_tot_va_callback)(ClcTotVa);
  uint16_t (*max_rmp_rte_callback)(void);
  void (*set_max_rmp_rte_callback)(uint16_t);
  uint16_t (*ecp_nom_hz_callback)(void);
  void (*set_ecp_nom_hz_callback)(uint16_t);
  ConnPh (*conn_ph_callback)(void);
  void (*set_conn_ph_callback)(ConnPh);
  uint16_t (*w_max_sf_callback)(void);
  uint16_t (*v_ref_sf_callback)(void);
  uint16_t (*v_ref_ofs_sf_callback)(void);
  uint16_t (*v_min_max_sf_callback)(void);
  uint16_t (*va_max_sf_callback)(void);
  uint16_t (*v_ar_max_sf_callback)(void);
  uint16_t (*w_gra_sf_callback)(void);
  uint16_t (*pf_min_sf_callback)(void);
  uint16_t (*max_rmp_rte_sf_callback)(void);
  uint16_t (*ecp_nom_hz_sf_callback)(void);
} Model121CallbackAdapter;

typedef struct Model122CallbackAdapter {
  uint16_t (*pv_conn_callback)(void);
  uint16_t (*stor_conn_callback)(void);
  uint16_t (*ecp_conn_callback)(void);
  uint64_t (*act_wh_callback)(void);
  uint64_t (*act_v_ah_callback)(void);
  uint64_t (*act_v_arh_q1_callback)(void);
  uint64_t (*act_v_arh_q2_callback)(void);
  uint64_t (*act_v_arh_q3_callback)(void);
  uint64_t (*act_v_arh_q4_callback)(void);
  int16_t (*v_ar_aval_callback)(void);
  uint16_t (*v_ar_aval_sf_callback)(void);
  uint16_t (*w_aval_callback)(void);
  uint16_t (*w_aval_sf_callback)(void);
  uint32_t (*st_set_lim_msk_callback)(void);
  uint32_t (*st_act_ctl_callback)(void);
  const char *(*tm_src_callback)(void);
  uint32_t (*tms_callback)(void);
  uint16_t (*rt_st_callback)(void);
  uint16_t (*ris_callback)(void);
  uint16_t (*ris_sf_callback)(void);
} Model122CallbackAdapter;

typedef struct Model123CallbackAdapter {
  uint16_t (*conn_win_tms_callback)(void);
  void (*set_conn_win_tms_callback)(uint16_t);
  uint16_t (*conn_rvrt_tms_callback)(void);
  void (*set_conn_rvrt_tms_callback)(uint16_t);
  Conn (*conn_callback)(void);
  void (*set_conn_callback)(Conn);
  uint16_t (*w_max_lim_pct_callback)(void);
  void (*set_w_max_lim_pct_callback)(uint16_t);
  uint16_t (*w_max_lim_pct_win_tms_callback)(void);
  void (*set_w_max_lim_pct_win_tms_callback)(uint16_t);
  uint16_t (*w_max_lim_pct_rvrt_tms_callback)(void);
  void (*set_w_max_lim_pct_rvrt_tms_callback)(uint16_t);
  uint16_t (*w_max_lim_pct_rmp_tms_callback)(void);
  void (*set_w_max_lim_pct_rmp_tms_callback)(uint16_t);
  WMaxLimEna (*w_max_lim_ena_callback)(void);
  void (*set_w_max_lim_ena_callback)(WMaxLimEna);
  int16_t (*out_pf_set_callback)(void);
  void (*set_out_pf_set_callback)(int16_t);
  uint16_t (*out_pf_set_win_tms_callback)(void);
  void (*set_out_pf_set_win_tms_callback)(uint16_t);
  uint16_t (*out_pf_set_rvrt_tms_callback)(void);
  void (*set_out_pf_set_rvrt_tms_callback)(uint16_t);
  uint16_t (*out_pf_set_rmp_tms_callback)(void);
  void (*set_out_pf_set_rmp_tms_callback)(uint16_t);
  OutPfSetEna (*out_pf_set_ena_callback)(void);
  void (*set_out_pf_set_ena_callback)(OutPfSetEna);
  int16_t (*v_ar_w_max_pct_callback)(void);
  void (*set_v_ar_w_max_pct_callback)(int16_t);
  int16_t (*v_ar_max_pct_callback)(void);
  void (*set_v_ar_max_pct_callback)(int16_t);
  int16_t (*v_ar_aval_pct_callback)(void);
  void (*set_v_ar_aval_pct_callback)(int16_t);
  uint16_t (*v_ar_pct_win_tms_callback)(void);
  void (*set_v_ar_pct_win_tms_callback)(uint16_t);
  uint16_t (*v_ar_pct_rvrt_tms_callback)(void);
  void (*set_v_ar_pct_rvrt_tms_callback)(uint16_t);
  uint16_t (*v_ar_pct_rmp_tms_callback)(void);
  void (*set_v_ar_pct_rmp_tms_callback)(uint16_t);
  VArPctMod (*v_ar_pct_mod_callback)(void);
  void (*set_v_ar_pct_mod_callback)(VArPctMod);
  VArPctEna (*v_ar_pct_ena_callback)(void);
  void (*set_v_ar_pct_ena_callback)(VArPctEna);
  uint16_t (*w_max_lim_pct_sf_callback)(void);
  uint16_t (*out_pf_set_sf_callback)(void);
  uint16_t (*v_ar_pct_sf_callback)(void);
} Model123CallbackAdapter;

typedef struct Model124CallbackAdapter {
  uint16_t (*w_cha_max_callback)(void);
  void (*set_w_cha_max_callback)(uint16_t);
  uint16_t (*w_cha_gra_callback)(void);
  void (*set_w_cha_gra_callback)(uint16_t);
  uint16_t (*w_dis_cha_gra_callback)(void);
  void (*set_w_dis_cha_gra_callback)(uint16_t);
  uint16_t (*stor_ctl_mod_callback)(void);
  void (*set_stor_ctl_mod_callback)(uint16_t);
  uint16_t (*va_cha_max_callback)(void);
  void (*set_va_cha_max_callback)(uint16_t);
  uint16_t (*min_rsv_pct_callback)(void);
  void (*set_min_rsv_pct_callback)(uint16_t);
  uint16_t (*cha_state_callback)(void);
  uint16_t (*stor_aval_callback)(void);
  uint16_t (*in_bat_v_callback)(void);
  ChaSt (*cha_st_callback)(void);
  int16_t (*out_w_rte_callback)(void);
  void (*set_out_w_rte_callback)(int16_t);
  int16_t (*in_w_rte_callback)(void);
  void (*set_in_w_rte_callback)(int16_t);
  uint16_t (*in_out_w_rte_win_tms_callback)(void);
  void (*set_in_out_w_rte_win_tms_callback)(uint16_t);
  uint16_t (*in_out_w_rte_rvrt_tms_callback)(void);
  void (*set_in_out_w_rte_rvrt_tms_callback)(uint16_t);
  uint16_t (*in_out_w_rte_rmp_tms_callback)(void);
  void (*set_in_out_w_rte_rmp_tms_callback)(uint16_t);
  ChaGriSet (*cha_gri_set_callback)(void);
  void (*set_cha_gri_set_callback)(ChaGriSet);
  uint16_t (*w_cha_max_sf_callback)(void);
  uint16_t (*w_cha_dis_cha_gra_sf_callback)(void);
  uint16_t (*va_cha_max_sf_callback)(void);
  uint16_t (*min_rsv_pct_sf_callback)(void);
  uint16_t (*cha_state_sf_callback)(void);
  uint16_t (*stor_aval_sf_callback)(void);
  uint16_t (*in_bat_v_sf_callback)(void);
  uint16_t (*in_out_w_rte_sf_callback)(void);
} Model124CallbackAdapter;

typedef struct Model125CallbackAdapter {
  uint16_t (*mod_ena_callback)(void);
  void (*set_mod_ena_callback)(uint16_t);
  SigType (*sig_type_callback)(void);
  void (*set_sig_type_callback)(SigType);
  int16_t (*sig_callback)(void);
  void (*set_sig_callback)(int16_t);
  uint16_t (*win_tms_callback)(void);
  void (*set_win_tms_callback)(uint16_t);
  uint16_t (*rvt_tms_callback)(void);
  void (*set_rvt_tms_callback)(uint16_t);
  uint16_t (*rmp_tms_callback)(void);
  void (*set_rmp_tms_callback)(uint16_t);
  uint16_t (*sig_sf_callback)(void);
} Model125CallbackAdapter;

typedef struct Model126CallbackAdapter {
  uint16_t (*act_crv_callback)(void);
  void (*set_act_crv_callback)(uint16_t);
  uint16_t (*mod_ena_callback)(void);
  void (*set_mod_ena_callback)(uint16_t);
  uint16_t (*win_tms_callback)(void);
  void (*set_win_tms_callback)(uint16_t);
  uint16_t (*rvrt_tms_callback)(void);
  void (*set_rvrt_tms_callback)(uint16_t);
  uint16_t (*rmp_tms_callback)(void);
  void (*set_rmp_tms_callback)(uint16_t);
  uint16_t (*n_crv_callback)(void);
  uint16_t (*n_pt_callback)(void);
  uint16_t (*v_sf_callback)(void);
  uint16_t (*dept_ref_sf_callback)(void);
  uint16_t (*rmp_inc_dec_sf_callback)(void);
} Model126CallbackAdapter;

typedef struct Model127CallbackAdapter {
  uint16_t (*w_gra_callback)(void);
  void (*set_w_gra_callback)(uint16_t);
  int16_t (*hz_str_callback)(void);
  void (*set_hz_str_callback)(int16_t);
  int16_t (*hz_stop_callback)(void);
  void (*set_hz_stop_callback)(int16_t);
  uint16_t (*hys_ena_callback)(void);
  void (*set_hys_ena_callback)(uint16_t);
  uint16_t (*mod_ena_callback)(void);
  void (*set_mod_ena_callback)(uint16_t);
  uint16_t (*hz_stop_w_gra_callback)(void);
  void (*set_hz_stop_w_gra_callback)(uint16_t);
  uint16_t (*w_gra_sf_callback)(void);
  uint16_t (*hz_str_stop_sf_callback)(void);
  uint16_t (*rmp_inc_dec_sf_callback)(void);
} Model127CallbackAdapter;

typedef struct Model128CallbackAdapter {
  ArGraMod (*ar_gra_mod_callback)(void);
  void (*set_ar_gra_mod_callback)(ArGraMod);
  uint16_t (*ar_gra_sag_callback)(void);
  void (*set_ar_gra_sag_callback)(uint16_t);
  uint16_t (*ar_gra_swell_callback)(void);
  void (*set_ar_gra_swell_callback)(uint16_t);
  uint16_t (*mod_ena_callback)(void);
  void (*set_mod_ena_callback)(uint16_t);
  uint16_t (*fil_tms_callback)(void);
  void (*set_fil_tms_callback)(uint16_t);
  uint16_t (*db_v_min_callback)(void);
  void (*set_db_v_min_callback)(uint16_t);
  uint16_t (*db_v_max_callback)(void);
  void (*set_db_v_max_callback)(uint16_t);
  uint16_t (*blk_zn_v_callback)(void);
  void (*set_blk_zn_v_callback)(uint16_t);
  uint16_t (*hys_blk_zn_v_callback)(void);
  void (*set_hys_blk_zn_v_callback)(uint16_t);
  uint16_t (*blk_zn_tmms_callback)(void);
  void (*set_blk_zn_tmms_callback)(uint16_t);
  uint16_t (*hold_tmms_callback)(void);
  void (*set_hold_tmms_callback)(uint16_t);
  uint16_t (*ar_gra_sf_callback)(void);
  uint16_t (*v_ref_pct_sf_callback)(void);
} Model128CallbackAdapter;

typedef struct Model129CallbackAdapter {
  uint16_t (*act_crv_callback)(void);
  void (*set_act_crv_callback)(uint16_t);
  uint16_t (*mod_ena_callback)(void);
  void (*set_mod_ena_callback)(uint16_t);
  uint16_t (*win_tms_callback)(void);
  void (*set_win_tms_callback)(uint16_t);
  uint16_t (*rvrt_tms_callback)(void);
  void (*set_rvrt_tms_callback)(uint16_t);
  uint16_t (*rmp_tms_callback)(void);
  void (*set_rmp_tms_callback)(uint16_t);
  uint16_t (*n_crv_callback)(void);
  uint16_t (*n_pt_callback)(void);
  uint16_t (*tms_sf_callback)(void);
  uint16_t (*v_sf_callback)(void);
} Model129CallbackAdapter;

typedef struct Model13CallbackAdapter {
  const char *(*name_callback)(void);
  void (*set_name_callback)(const char*);
  CfgSt (*config_status_callback)(void);
  uint16_t (*change_status_callback)(void);
  uint16_t (*config_capability_callback)(void);
  Cfg (*i_pv6_config_callback)(void);
  void (*set_i_pv6_config_callback)(Cfg);
  Ctl (*control_callback)(void);
  void (*set_control_callback)(Ctl);
  const char *(*ip_callback)(void);
  void (*set_ip_callback)(const char*);
  const char *(*cidr_callback)(void);
  void (*set_cidr_callback)(const char*);
  const char *(*gateway_callback)(void);
  void (*set_gateway_callback)(const char*);
  const char *(*dns1_callback)(void);
  void (*set_dns1_callback)(const char*);
  const char *(*dns2_callback)(void);
  void (*set_dns2_callback)(const char*);
  const char *(*ntp1_callback)(void);
  void (*set_ntp1_callback)(const char*);
  const char *(*ntp2_callback)(void);
  void (*set_ntp2_callback)(const char*);
  const char *(*domain_callback)(void);
  void (*set_domain_callback)(const char*);
  const char *(*host_name_callback)(void);
  void (*set_host_name_callback)(const char*);
} Model13CallbackAdapter;

typedef struct Model130CallbackAdapter {
  uint16_t (*act_crv_callback)(void);
  void (*set_act_crv_callback)(uint16_t);
  uint16_t (*mod_ena_callback)(void);
  void (*set_mod_ena_callback)(uint16_t);
  uint16_t (*win_tms_callback)(void);
  void (*set_win_tms_callback)(uint16_t);
  uint16_t (*rvrt_tms_callback)(void);
  void (*set_rvrt_tms_callback)(uint16_t);
  uint16_t (*rmp_tms_callback)(void);
  void (*set_rmp_tms_callback)(uint16_t);
  uint16_t (*n_crv_callback)(void);
  uint16_t (*n_pt_callback)(void);
  uint16_t (*tms_sf_callback)(void);
  uint16_t (*v_sf_callback)(void);
} Model130CallbackAdapter;

typedef struct Model131CallbackAdapter {
  uint16_t (*act_crv_callback)(void);
  void (*set_act_crv_callback)(uint16_t);
  uint16_t (*mod_ena_callback)(void);
  void (*set_mod_ena_callback)(uint16_t);
  uint16_t (*win_tms_callback)(void);
  void (*set_win_tms_callback)(uint16_t);
  uint16_t (*rvrt_tms_callback)(void);
  void (*set_rvrt_tms_callback)(uint16_t);
  uint16_t (*rmp_tms_callback)(void);
  void (*set_rmp_tms_callback)(uint16_t);
  uint16_t (*n_crv_callback)(void);
  uint16_t (*n_pt_callback)(void);
  uint16_t (*w_sf_callback)(void);
  uint16_t (*pf_sf_callback)(void);
  uint16_t (*rmp_inc_dec_sf_callback)(void);
} Model131CallbackAdapter;

typedef struct Model132CallbackAdapter {
  uint16_t (*act_crv_callback)(void);
  void (*set_act_crv_callback)(uint16_t);
  uint16_t (*mod_ena_callback)(void);
  void (*set_mod_ena_callback)(uint16_t);
  uint16_t (*win_tms_callback)(void);
  void (*set_win_tms_callback)(uint16_t);
  uint16_t (*rvrt_tms_callback)(void);
  void (*set_rvrt_tms_callback)(uint16_t);
  uint16_t (*rmp_tms_callback)(void);
  void (*set_rmp_tms_callback)(uint16_t);
  uint16_t (*n_crv_callback)(void);
  uint16_t (*n_pt_callback)(void);
  uint16_t (*v_sf_callback)(void);
  uint16_t (*dept_ref_sf_callback)(void);
  uint16_t (*rmp_inc_dec_sf_callback)(void);
} Model132CallbackAdapter;

typedef struct Model133CallbackAdapter {
  uint32_t (*act_schd_callback)(void);
  void (*set_act_schd_callback)(uint32_t);
  uint16_t (*mod_ena_callback)(void);
  void (*set_mod_ena_callback)(uint16_t);
  uint16_t (*n_schd_callback)(void);
  uint16_t (*n_pts_callback)(void);
} Model133CallbackAdapter;

typedef struct Model134CallbackAdapter {
  uint16_t (*act_crv_callback)(void);
  void (*set_act_crv_callback)(uint16_t);
  uint16_t (*mod_ena_callback)(void);
  void (*set_mod_ena_callback)(uint16_t);
  uint16_t (*win_tms_callback)(void);
  void (*set_win_tms_callback)(uint16_t);
  uint16_t (*rvrt_tms_callback)(void);
  void (*set_rvrt_tms_callback)(uint16_t);
  uint16_t (*rmp_tms_callback)(void);
  void (*set_rmp_tms_callback)(uint16_t);
  uint16_t (*n_crv_callback)(void);
  uint16_t (*n_pt_callback)(void);
  uint16_t (*hz_sf_callback)(void);
  uint16_t (*w_sf_callback)(void);
  uint16_t (*rmp_inc_dec_sf_callback)(void);
} Model134CallbackAdapter;

typedef struct Model135CallbackAdapter {
  uint16_t (*act_crv_callback)(void);
  void (*set_act_crv_callback)(uint16_t);
  uint16_t (*mod_ena_callback)(void);
  void (*set_mod_ena_callback)(uint16_t);
  uint16_t (*win_tms_callback)(void);
  void (*set_win_tms_callback)(uint16_t);
  uint16_t (*rvrt_tms_callback)(void);
  void (*set_rvrt_tms_callback)(uint16_t);
  uint16_t (*rmp_tms_callback)(void);
  void (*set_rmp_tms_callback)(uint16_t);
  uint16_t (*n_crv_callback)(void);
  uint16_t (*n_pt_callback)(void);
  uint16_t (*tms_sf_callback)(void);
  uint16_t (*hz_sf_callback)(void);
} Model135CallbackAdapter;

typedef struct Model136CallbackAdapter {
  uint16_t (*act_crv_callback)(void);
  void (*set_act_crv_callback)(uint16_t);
  uint16_t (*mod_ena_callback)(void);
  void (*set_mod_ena_callback)(uint16_t);
  uint16_t (*win_tms_callback)(void);
  void (*set_win_tms_callback)(uint16_t);
  uint16_t (*rvrt_tms_callback)(void);
  void (*set_rvrt_tms_callback)(uint16_t);
  uint16_t (*rmp_tms_callback)(void);
  void (*set_rmp_tms_callback)(uint16_t);
  uint16_t (*n_crv_callback)(void);
  uint16_t (*n_pt_callback)(void);
  uint16_t (*tms_sf_callback)(void);
  uint16_t (*hz_sf_callback)(void);
} Model136CallbackAdapter;

typedef struct Model137CallbackAdapter {
  uint16_t (*act_crv_callback)(void);
  void (*set_act_crv_callback)(uint16_t);
  uint16_t (*mod_ena_callback)(void);
  void (*set_mod_ena_callback)(uint16_t);
  uint16_t (*win_tms_callback)(void);
  void (*set_win_tms_callback)(uint16_t);
  uint16_t (*rvrt_tms_callback)(void);
  void (*set_rvrt_tms_callback)(uint16_t);
  uint16_t (*rmp_tms_callback)(void);
  void (*set_rmp_tms_callback)(uint16_t);
  uint16_t (*n_crv_callback)(void);
  uint16_t (*n_pt_callback)(void);
  uint16_t (*tms_sf_callback)(void);
  uint16_t (*v_sf_callback)(void);
} Model137CallbackAdapter;

typedef struct Model138CallbackAdapter {
  uint16_t (*act_crv_callback)(void);
  void (*set_act_crv_callback)(uint16_t);
  uint16_t (*mod_ena_callback)(void);
  void (*set_mod_ena_callback)(uint16_t);
  uint16_t (*win_tms_callback)(void);
  void (*set_win_tms_callback)(uint16_t);
  uint16_t (*rvrt_tms_callback)(void);
  void (*set_rvrt_tms_callback)(uint16_t);
  uint16_t (*rmp_tms_callback)(void);
  void (*set_rmp_tms_callback)(uint16_t);
  uint16_t (*n_crv_callback)(void);
  uint16_t (*n_pt_callback)(void);
  uint16_t (*tms_sf_callback)(void);
  uint16_t (*v_sf_callback)(void);
} Model138CallbackAdapter;

typedef struct Model139CallbackAdapter {
  uint16_t (*act_crv_callback)(void);
  void (*set_act_crv_callback)(uint16_t);
  uint16_t (*mod_ena_callback)(void);
  void (*set_mod_ena_callback)(uint16_t);
  uint16_t (*win_tms_callback)(void);
  void (*set_win_tms_callback)(uint16_t);
  uint16_t (*rvrt_tms_callback)(void);
  void (*set_rvrt_tms_callback)(uint16_t);
  uint16_t (*rmp_tms_callback)(void);
  void (*set_rmp_tms_callback)(uint16_t);
  uint16_t (*n_crv_callback)(void);
  uint16_t (*n_pt_callback)(void);
  uint16_t (*tms_sf_callback)(void);
  uint16_t (*v_sf_callback)(void);
  CrvType (*crv_type_callback)(void);
} Model139CallbackAdapter;

typedef struct Model140CallbackAdapter {
  uint16_t (*act_crv_callback)(void);
  void (*set_act_crv_callback)(uint16_t);
  uint16_t (*mod_ena_callback)(void);
  void (*set_mod_ena_callback)(uint16_t);
  uint16_t (*win_tms_callback)(void);
  void (*set_win_tms_callback)(uint16_t);
  uint16_t (*rvrt_tms_callback)(void);
  void (*set_rvrt_tms_callback)(uint16_t);
  uint16_t (*rmp_tms_callback)(void);
  void (*set_rmp_tms_callback)(uint16_t);
  uint16_t (*n_crv_callback)(void);
  uint16_t (*n_pt_callback)(void);
  uint16_t (*tms_sf_callback)(void);
  uint16_t (*v_sf_callback)(void);
  CrvType (*crv_type_callback)(void);
} Model140CallbackAdapter;

typedef struct Model141CallbackAdapter {
  uint16_t (*act_crv_callback)(void);
  void (*set_act_crv_callback)(uint16_t);
  uint16_t (*mod_ena_callback)(void);
  void (*set_mod_ena_callback)(uint16_t);
  uint16_t (*win_tms_callback)(void);
  void (*set_win_tms_callback)(uint16_t);
  uint16_t (*rvrt_tms_callback)(void);
  void (*set_rvrt_tms_callback)(uint16_t);
  uint16_t (*rmp_tms_callback)(void);
  void (*set_rmp_tms_callback)(uint16_t);
  uint16_t (*n_crv_callback)(void);
  uint16_t (*n_pt_callback)(void);
  uint16_t (*tms_sf_callback)(void);
  uint16_t (*hz_sf_callback)(void);
} Model141CallbackAdapter;

typedef struct Model142CallbackAdapter {
  uint16_t (*act_crv_callback)(void);
  void (*set_act_crv_callback)(uint16_t);
  uint16_t (*mod_ena_callback)(void);
  void (*set_mod_ena_callback)(uint16_t);
  uint16_t (*win_tms_callback)(void);
  void (*set_win_tms_callback)(uint16_t);
  uint16_t (*rvrt_tms_callback)(void);
  void (*set_rvrt_tms_callback)(uint16_t);
  uint16_t (*rmp_tms_callback)(void);
  void (*set_rmp_tms_callback)(uint16_t);
  uint16_t (*n_crv_callback)(void);
  uint16_t (*n_pt_callback)(void);
  uint16_t (*tms_sf_callback)(void);
  uint16_t (*hz_sf_callback)(void);
} Model142CallbackAdapter;

typedef struct Model143CallbackAdapter {
  uint16_t (*act_crv_callback)(void);
  void (*set_act_crv_callback)(uint16_t);
  uint16_t (*mod_ena_callback)(void);
  void (*set_mod_ena_callback)(uint16_t);
  uint16_t (*win_tms_callback)(void);
  void (*set_win_tms_callback)(uint16_t);
  uint16_t (*rvrt_tms_callback)(void);
  void (*set_rvrt_tms_callback)(uint16_t);
  uint16_t (*rmp_tms_callback)(void);
  void (*set_rmp_tms_callback)(uint16_t);
  uint16_t (*n_crv_callback)(void);
  uint16_t (*n_pt_callback)(void);
  uint16_t (*tms_sf_callback)(void);
  uint16_t (*hz_sf_callback)(void);
  CrvType (*crv_type_callback)(void);
} Model143CallbackAdapter;

typedef struct Model144CallbackAdapter {
  uint16_t (*act_crv_callback)(void);
  void (*set_act_crv_callback)(uint16_t);
  uint16_t (*mod_ena_callback)(void);
  void (*set_mod_ena_callback)(uint16_t);
  uint16_t (*win_tms_callback)(void);
  void (*set_win_tms_callback)(uint16_t);
  uint16_t (*rvrt_tms_callback)(void);
  void (*set_rvrt_tms_callback)(uint16_t);
  uint16_t (*rmp_tms_callback)(void);
  void (*set_rmp_tms_callback)(uint16_t);
  uint16_t (*n_crv_callback)(void);
  uint16_t (*n_pt_callback)(void);
  uint16_t (*tms_sf_callback)(void);
  uint16_t (*hz_sf_callback)(void);
  CrvType (*crv_type_callback)(void);
} Model144CallbackAdapter;

typedef struct Model145CallbackAdapter {
  uint16_t (*ramp_up_rate_callback)(void);
  void (*set_ramp_up_rate_callback)(uint16_t);
  uint16_t (*nom_rmp_dn_rte_callback)(void);
  void (*set_nom_rmp_dn_rte_callback)(uint16_t);
  uint16_t (*emergency_ramp_up_rate_callback)(void);
  void (*set_emergency_ramp_up_rate_callback)(uint16_t);
  uint16_t (*emergency_ramp_down_rate_callback)(void);
  void (*set_emergency_ramp_down_rate_callback)(uint16_t);
  uint16_t (*connect_ramp_up_rate_callback)(void);
  void (*set_connect_ramp_up_rate_callback)(uint16_t);
  uint16_t (*connect_ramp_down_rate_callback)(void);
  void (*set_connect_ramp_down_rate_callback)(uint16_t);
  uint16_t (*default_ramp_rate_callback)(void);
  void (*set_default_ramp_rate_callback)(uint16_t);
  uint16_t (*ramp_rate_scale_factor_callback)(void);
} Model145CallbackAdapter;

typedef struct Model15CallbackAdapter {
  uint16_t (*clear_callback)(void);
  void (*set_clear_callback)(uint16_t);
  uint32_t (*input_count_callback)(void);
  uint32_t (*input_unicast_count_callback)(void);
  uint32_t (*input_non_unicast_count_callback)(void);
  uint32_t (*input_discarded_count_callback)(void);
  uint32_t (*input_error_count_callback)(void);
  uint32_t (*input_unknown_count_callback)(void);
  uint32_t (*output_count_callback)(void);
  uint32_t (*output_unicast_count_callback)(void);
  uint32_t (*output_non_unicast_count_callback)(void);
  uint32_t (*output_discarded_count_callback)(void);
  uint32_t (*output_error_count_callback)(void);
} Model15CallbackAdapter;

typedef struct Model16CallbackAdapter {
  const char *(*name_callback)(void);
  void (*set_name_callback)(const char*);
  Cfg (*config_callback)(void);
  uint16_t (*control_callback)(void);
  void (*set_control_callback)(uint16_t);
  const char *(*address_callback)(void);
  void (*set_address_callback)(const char*);
  const char *(*netmask_callback)(void);
  void (*set_netmask_callback)(const char*);
  const char *(*gateway_callback)(void);
  void (*set_gateway_callback)(const char*);
  const char *(*dns1_callback)(void);
  void (*set_dns1_callback)(const char*);
  const char *(*dns2_callback)(void);
  void (*set_dns2_callback)(const char*);
  const uint8_t *(*mac_callback)(void);
  uint16_t (*link_control_callback)(void);
  void (*set_link_control_callback)(uint16_t);
} Model16CallbackAdapter;

typedef struct Model160CallbackAdapter {
  uint16_t (*current_scale_factor_callback)(void);
  uint16_t (*voltage_scale_factor_callback)(void);
  uint16_t (*power_scale_factor_callback)(void);
  uint16_t (*energy_scale_factor_callback)(void);
  uint32_t (*global_events_callback)(void);
  uint16_t (*number_of_modules_callback)(void);
  uint16_t (*timestamp_period_callback)(void);
} Model160CallbackAdapter;

typedef struct Model17CallbackAdapter {
  const char *(*name_callback)(void);
  void (*set_name_callback)(const char*);
  uint32_t (*rate_callback)(void);
  void (*set_rate_callback)(uint32_t);
  uint16_t (*bits_callback)(void);
  void (*set_bits_callback)(uint16_t);
  Pty (*parity_callback)(void);
  void (*set_parity_callback)(Pty);
  Dup (*duplex_callback)(void);
  void (*set_duplex_callback)(Dup);
  Flw (*flow_control_callback)(void);
  void (*set_flow_control_callback)(Flw);
  Typ (*interface_type_callback)(void);
  Pcol (*protocol_callback)(void);
} Model17CallbackAdapter;

typedef struct Model18CallbackAdapter {
  const char *(*name_callback)(void);
  void (*set_name_callback)(const char*);
  uint32_t (*imei_callback)(void);
  void (*set_imei_callback)(uint32_t);
  const char *(*apn_callback)(void);
  void (*set_apn_callback)(const char*);
  const char *(*number_callback)(void);
  void (*set_number_callback)(const char*);
  const char *(*pin_callback)(void);
  void (*set_pin_callback)(const char*);
} Model18CallbackAdapter;

typedef struct Model19CallbackAdapter {
  const char *(*name_callback)(void);
  void (*set_name_callback)(const char*);
  uint32_t (*rate_callback)(void);
  void (*set_rate_callback)(uint32_t);
  uint16_t (*bits_callback)(void);
  void (*set_bits_callback)(uint16_t);
  Pty (*parity_callback)(void);
  void (*set_parity_callback)(Pty);
  Dup (*duplex_callback)(void);
  void (*set_duplex_callback)(Dup);
  Flw (*flow_control_callback)(void);
  void (*set_flow_control_callback)(Flw);
  Auth (*authentication_callback)(void);
  const char *(*username_callback)(void);
  const char *(*password_callback)(void);
} Model19CallbackAdapter;

typedef struct Model2CallbackAdapter {
  uint16_t (*aid_callback)(void);
  uint16_t (*n_callback)(void);
  uint16_t (*un_callback)(void);
  St (*status_callback)(void);
  uint16_t (*vendor_status_callback)(void);
  uint32_t (*event_code_callback)(void);
  uint32_t (*vendor_event_code_callback)(void);
  Ctl (*control_callback)(void);
  uint32_t (*vendor_control_callback)(void);
  uint32_t (*control_value_callback)(void);
} Model2CallbackAdapter;

typedef struct Model201CallbackAdapter {
  int16_t (*amps_callback)(void);
  int16_t (*amps_phase_a_callback)(void);
  int16_t (*amps_phase_b_callback)(void);
  int16_t (*amps_phase_c_callback)(void);
  uint16_t (*a_sf_callback)(void);
  int16_t (*voltage_ln_callback)(void);
  int16_t (*phase_voltage_an_callback)(void);
  int16_t (*phase_voltage_bn_callback)(void);
  int16_t (*phase_voltage_cn_callback)(void);
  int16_t (*voltage_ll_callback)(void);
  int16_t (*phase_voltage_ab_callback)(void);
  int16_t (*phase_voltage_bc_callback)(void);
  int16_t (*phase_voltage_ca_callback)(void);
  uint16_t (*v_sf_callback)(void);
  int16_t (*hz_callback)(void);
  uint16_t (*hz_sf_callback)(void);
  int16_t (*watts_callback)(void);
  int16_t (*watts_phase_a_callback)(void);
  int16_t (*watts_phase_b_callback)(void);
  int16_t (*watts_phase_c_callback)(void);
  uint16_t (*w_sf_callback)(void);
  int16_t (*va_callback)(void);
  int16_t (*va_phase_a_callback)(void);
  int16_t (*va_phase_b_callback)(void);
  int16_t (*va_phase_c_callback)(void);
  uint16_t (*va_sf_callback)(void);
  int16_t (*var_callback)(void);
  int16_t (*var_phase_a_callback)(void);
  int16_t (*var_phase_b_callback)(void);
  int16_t (*var_phase_c_callback)(void);
  uint16_t (*var_sf_callback)(void);
  int16_t (*pf_callback)(void);
  int16_t (*pf_phase_a_callback)(void);
  int16_t (*pf_phase_b_callback)(void);
  int16_t (*pf_phase_c_callback)(void);
  uint16_t (*pf_sf_callback)(void);
  uint32_t (*total_watt_hours_exported_callback)(void);
  uint32_t (*total_watt_hours_exported_phase_a_callback)(void);
  uint32_t (*total_watt_hours_exported_phase_b_callback)(void);
  uint32_t (*total_watt_hours_exported_phase_c_callback)(void);
  uint32_t (*total_watt_hours_imported_callback)(void);
  uint32_t (*total_watt_hours_imported_phase_a_callback)(void);
  uint32_t (*total_watt_hours_imported_phase_b_callback)(void);
  uint32_t (*total_watt_hours_imported_phase_c_callback)(void);
  uint16_t (*tot_wh_sf_callback)(void);
  uint32_t (*total_va_hours_exported_callback)(void);
  uint32_t (*total_va_hours_exported_phase_a_callback)(void);
  uint32_t (*total_va_hours_exported_phase_b_callback)(void);
  uint32_t (*total_va_hours_exported_phase_c_callback)(void);
  uint32_t (*total_va_hours_imported_callback)(void);
  uint32_t (*total_va_hours_imported_phase_a_callback)(void);
  uint32_t (*total_va_hours_imported_phase_b_callback)(void);
  uint32_t (*total_va_hours_imported_phase_c_callback)(void);
  uint16_t (*tot_v_ah_sf_callback)(void);
  uint32_t (*total_var_hours_imported_q1_callback)(void);
  uint32_t (*total_v_ar_hours_imported_q1_phase_a_callback)(void);
  uint32_t (*total_v_ar_hours_imported_q1_phase_b_callback)(void);
  uint32_t (*total_v_ar_hours_imported_q1_phase_c_callback)(void);
  uint32_t (*total_v_ar_hours_imported_q2_callback)(void);
  uint32_t (*total_v_ar_hours_imported_q2_phase_a_callback)(void);
  uint32_t (*total_v_ar_hours_imported_q2_phase_b_callback)(void);
  uint32_t (*total_v_ar_hours_imported_q2_phase_c_callback)(void);
  uint32_t (*total_v_ar_hours_exported_q3_callback)(void);
  uint32_t (*total_v_ar_hours_exported_q3_phase_a_callback)(void);
  uint32_t (*total_v_ar_hours_exported_q3_phase_b_callback)(void);
  uint32_t (*total_v_ar_hours_exported_q3_phase_c_callback)(void);
  uint32_t (*total_v_ar_hours_exported_q4_callback)(void);
  uint32_t (*total_v_ar_hours_exported_q4_imported_phase_a_callback)(void);
  uint32_t (*total_v_ar_hours_exported_q4_imported_phase_b_callback)(void);
  uint32_t (*total_v_ar_hours_exported_q4_imported_phase_c_callback)(void);
  uint16_t (*tot_v_arh_sf_callback)(void);
  uint32_t (*events_callback)(void);
} Model201CallbackAdapter;

typedef struct Model202CallbackAdapter {
  int16_t (*amps_callback)(void);
  int16_t (*amps_phase_a_callback)(void);
  int16_t (*amps_phase_b_callback)(void);
  int16_t (*amps_phase_c_callback)(void);
  uint16_t (*a_sf_callback)(void);
  int16_t (*voltage_ln_callback)(void);
  int16_t (*phase_voltage_an_callback)(void);
  int16_t (*phase_voltage_bn_callback)(void);
  int16_t (*phase_voltage_cn_callback)(void);
  int16_t (*voltage_ll_callback)(void);
  int16_t (*phase_voltage_ab_callback)(void);
  int16_t (*phase_voltage_bc_callback)(void);
  int16_t (*phase_voltage_ca_callback)(void);
  uint16_t (*v_sf_callback)(void);
  int16_t (*hz_callback)(void);
  uint16_t (*hz_sf_callback)(void);
  int16_t (*watts_callback)(void);
  int16_t (*watts_phase_a_callback)(void);
  int16_t (*watts_phase_b_callback)(void);
  int16_t (*watts_phase_c_callback)(void);
  uint16_t (*w_sf_callback)(void);
  int16_t (*va_callback)(void);
  int16_t (*va_phase_a_callback)(void);
  int16_t (*va_phase_b_callback)(void);
  int16_t (*va_phase_c_callback)(void);
  uint16_t (*va_sf_callback)(void);
  int16_t (*var_callback)(void);
  int16_t (*var_phase_a_callback)(void);
  int16_t (*var_phase_b_callback)(void);
  int16_t (*var_phase_c_callback)(void);
  uint16_t (*var_sf_callback)(void);
  int16_t (*pf_callback)(void);
  int16_t (*pf_phase_a_callback)(void);
  int16_t (*pf_phase_b_callback)(void);
  int16_t (*pf_phase_c_callback)(void);
  uint16_t (*pf_sf_callback)(void);
  uint32_t (*total_watt_hours_exported_callback)(void);
  uint32_t (*total_watt_hours_exported_phase_a_callback)(void);
  uint32_t (*total_watt_hours_exported_phase_b_callback)(void);
  uint32_t (*total_watt_hours_exported_phase_c_callback)(void);
  uint32_t (*total_watt_hours_imported_callback)(void);
  uint32_t (*total_watt_hours_imported_phase_a_callback)(void);
  uint32_t (*total_watt_hours_imported_phase_b_callback)(void);
  uint32_t (*total_watt_hours_imported_phase_c_callback)(void);
  uint16_t (*tot_wh_sf_callback)(void);
  uint32_t (*total_va_hours_exported_callback)(void);
  uint32_t (*total_va_hours_exported_phase_a_callback)(void);
  uint32_t (*total_va_hours_exported_phase_b_callback)(void);
  uint32_t (*total_va_hours_exported_phase_c_callback)(void);
  uint32_t (*total_va_hours_imported_callback)(void);
  uint32_t (*total_va_hours_imported_phase_a_callback)(void);
  uint32_t (*total_va_hours_imported_phase_b_callback)(void);
  uint32_t (*total_va_hours_imported_phase_c_callback)(void);
  uint16_t (*tot_v_ah_sf_callback)(void);
  uint32_t (*total_var_hours_imported_q1_callback)(void);
  uint32_t (*total_v_ar_hours_imported_q1_phase_a_callback)(void);
  uint32_t (*total_v_ar_hours_imported_q1_phase_b_callback)(void);
  uint32_t (*total_v_ar_hours_imported_q1_phase_c_callback)(void);
  uint32_t (*total_v_ar_hours_imported_q2_callback)(void);
  uint32_t (*total_v_ar_hours_imported_q2_phase_a_callback)(void);
  uint32_t (*total_v_ar_hours_imported_q2_phase_b_callback)(void);
  uint32_t (*total_v_ar_hours_imported_q2_phase_c_callback)(void);
  uint32_t (*total_v_ar_hours_exported_q3_callback)(void);
  uint32_t (*total_v_ar_hours_exported_q3_phase_a_callback)(void);
  uint32_t (*total_v_ar_hours_exported_q3_phase_b_callback)(void);
  uint32_t (*total_v_ar_hours_exported_q3_phase_c_callback)(void);
  uint32_t (*total_v_ar_hours_exported_q4_callback)(void);
  uint32_t (*total_v_ar_hours_exported_q4_imported_phase_a_callback)(void);
  uint32_t (*total_v_ar_hours_exported_q4_imported_phase_b_callback)(void);
  uint32_t (*total_v_ar_hours_exported_q4_imported_phase_c_callback)(void);
  uint16_t (*tot_v_arh_sf_callback)(void);
  uint32_t (*events_callback)(void);
} Model202CallbackAdapter;

typedef struct Model203CallbackAdapter {
  int16_t (*amps_callback)(void);
  int16_t (*amps_phase_a_callback)(void);
  int16_t (*amps_phase_b_callback)(void);
  int16_t (*amps_phase_c_callback)(void);
  uint16_t (*a_sf_callback)(void);
  int16_t (*voltage_ln_callback)(void);
  int16_t (*phase_voltage_an_callback)(void);
  int16_t (*phase_voltage_bn_callback)(void);
  int16_t (*phase_voltage_cn_callback)(void);
  int16_t (*voltage_ll_callback)(void);
  int16_t (*phase_voltage_ab_callback)(void);
  int16_t (*phase_voltage_bc_callback)(void);
  int16_t (*phase_voltage_ca_callback)(void);
  uint16_t (*v_sf_callback)(void);
  int16_t (*hz_callback)(void);
  uint16_t (*hz_sf_callback)(void);
  int16_t (*watts_callback)(void);
  int16_t (*watts_phase_a_callback)(void);
  int16_t (*watts_phase_b_callback)(void);
  int16_t (*watts_phase_c_callback)(void);
  uint16_t (*w_sf_callback)(void);
  int16_t (*va_callback)(void);
  int16_t (*va_phase_a_callback)(void);
  int16_t (*va_phase_b_callback)(void);
  int16_t (*va_phase_c_callback)(void);
  uint16_t (*va_sf_callback)(void);
  int16_t (*var_callback)(void);
  int16_t (*var_phase_a_callback)(void);
  int16_t (*var_phase_b_callback)(void);
  int16_t (*var_phase_c_callback)(void);
  uint16_t (*var_sf_callback)(void);
  int16_t (*pf_callback)(void);
  int16_t (*pf_phase_a_callback)(void);
  int16_t (*pf_phase_b_callback)(void);
  int16_t (*pf_phase_c_callback)(void);
  uint16_t (*pf_sf_callback)(void);
  uint32_t (*total_watt_hours_exported_callback)(void);
  uint32_t (*total_watt_hours_exported_phase_a_callback)(void);
  uint32_t (*total_watt_hours_exported_phase_b_callback)(void);
  uint32_t (*total_watt_hours_exported_phase_c_callback)(void);
  uint32_t (*total_watt_hours_imported_callback)(void);
  uint32_t (*total_watt_hours_imported_phase_a_callback)(void);
  uint32_t (*total_watt_hours_imported_phase_b_callback)(void);
  uint32_t (*total_watt_hours_imported_phase_c_callback)(void);
  uint16_t (*tot_wh_sf_callback)(void);
  uint32_t (*total_va_hours_exported_callback)(void);
  uint32_t (*total_va_hours_exported_phase_a_callback)(void);
  uint32_t (*total_va_hours_exported_phase_b_callback)(void);
  uint32_t (*total_va_hours_exported_phase_c_callback)(void);
  uint32_t (*total_va_hours_imported_callback)(void);
  uint32_t (*total_va_hours_imported_phase_a_callback)(void);
  uint32_t (*total_va_hours_imported_phase_b_callback)(void);
  uint32_t (*total_va_hours_imported_phase_c_callback)(void);
  uint16_t (*tot_v_ah_sf_callback)(void);
  uint32_t (*total_var_hours_imported_q1_callback)(void);
  uint32_t (*total_v_ar_hours_imported_q1_phase_a_callback)(void);
  uint32_t (*total_v_ar_hours_imported_q1_phase_b_callback)(void);
  uint32_t (*total_v_ar_hours_imported_q1_phase_c_callback)(void);
  uint32_t (*total_v_ar_hours_imported_q2_callback)(void);
  uint32_t (*total_v_ar_hours_imported_q2_phase_a_callback)(void);
  uint32_t (*total_v_ar_hours_imported_q2_phase_b_callback)(void);
  uint32_t (*total_v_ar_hours_imported_q2_phase_c_callback)(void);
  uint32_t (*total_v_ar_hours_exported_q3_callback)(void);
  uint32_t (*total_v_ar_hours_exported_q3_phase_a_callback)(void);
  uint32_t (*total_v_ar_hours_exported_q3_phase_b_callback)(void);
  uint32_t (*total_v_ar_hours_exported_q3_phase_c_callback)(void);
  uint32_t (*total_v_ar_hours_exported_q4_callback)(void);
  uint32_t (*total_v_ar_hours_exported_q4_imported_phase_a_callback)(void);
  uint32_t (*total_v_ar_hours_exported_q4_imported_phase_b_callback)(void);
  uint32_t (*total_v_ar_hours_exported_q4_imported_phase_c_callback)(void);
  uint16_t (*tot_v_arh_sf_callback)(void);
  uint32_t (*events_callback)(void);
} Model203CallbackAdapter;

typedef struct Model204CallbackAdapter {
  int16_t (*amps_callback)(void);
  int16_t (*amps_phase_a_callback)(void);
  int16_t (*amps_phase_b_callback)(void);
  int16_t (*amps_phase_c_callback)(void);
  uint16_t (*a_sf_callback)(void);
  int16_t (*voltage_ln_callback)(void);
  int16_t (*phase_voltage_an_callback)(void);
  int16_t (*phase_voltage_bn_callback)(void);
  int16_t (*phase_voltage_cn_callback)(void);
  int16_t (*voltage_ll_callback)(void);
  int16_t (*phase_voltage_ab_callback)(void);
  int16_t (*phase_voltage_bc_callback)(void);
  int16_t (*phase_voltage_ca_callback)(void);
  uint16_t (*v_sf_callback)(void);
  int16_t (*hz_callback)(void);
  uint16_t (*hz_sf_callback)(void);
  int16_t (*watts_callback)(void);
  int16_t (*watts_phase_a_callback)(void);
  int16_t (*watts_phase_b_callback)(void);
  int16_t (*watts_phase_c_callback)(void);
  uint16_t (*w_sf_callback)(void);
  int16_t (*va_callback)(void);
  int16_t (*va_phase_a_callback)(void);
  int16_t (*va_phase_b_callback)(void);
  int16_t (*va_phase_c_callback)(void);
  uint16_t (*va_sf_callback)(void);
  int16_t (*var_callback)(void);
  int16_t (*var_phase_a_callback)(void);
  int16_t (*var_phase_b_callback)(void);
  int16_t (*var_phase_c_callback)(void);
  uint16_t (*var_sf_callback)(void);
  int16_t (*pf_callback)(void);
  int16_t (*pf_phase_a_callback)(void);
  int16_t (*pf_phase_b_callback)(void);
  int16_t (*pf_phase_c_callback)(void);
  uint16_t (*pf_sf_callback)(void);
  uint32_t (*total_watt_hours_exported_callback)(void);
  uint32_t (*total_watt_hours_exported_phase_a_callback)(void);
  uint32_t (*total_watt_hours_exported_phase_b_callback)(void);
  uint32_t (*total_watt_hours_exported_phase_c_callback)(void);
  uint32_t (*total_watt_hours_imported_callback)(void);
  uint32_t (*total_watt_hours_imported_phase_a_callback)(void);
  uint32_t (*total_watt_hours_imported_phase_b_callback)(void);
  uint32_t (*total_watt_hours_imported_phase_c_callback)(void);
  uint16_t (*tot_wh_sf_callback)(void);
  uint32_t (*total_va_hours_exported_callback)(void);
  uint32_t (*total_va_hours_exported_phase_a_callback)(void);
  uint32_t (*total_va_hours_exported_phase_b_callback)(void);
  uint32_t (*total_va_hours_exported_phase_c_callback)(void);
  uint32_t (*total_va_hours_imported_callback)(void);
  uint32_t (*total_va_hours_imported_phase_a_callback)(void);
  uint32_t (*total_va_hours_imported_phase_b_callback)(void);
  uint32_t (*total_va_hours_imported_phase_c_callback)(void);
  uint16_t (*tot_v_ah_sf_callback)(void);
  uint32_t (*total_var_hours_imported_q1_callback)(void);
  uint32_t (*total_v_ar_hours_imported_q1_phase_a_callback)(void);
  uint32_t (*total_v_ar_hours_imported_q1_phase_b_callback)(void);
  uint32_t (*total_v_ar_hours_imported_q1_phase_c_callback)(void);
  uint32_t (*total_v_ar_hours_imported_q2_callback)(void);
  uint32_t (*total_v_ar_hours_imported_q2_phase_a_callback)(void);
  uint32_t (*total_v_ar_hours_imported_q2_phase_b_callback)(void);
  uint32_t (*total_v_ar_hours_imported_q2_phase_c_callback)(void);
  uint32_t (*total_v_ar_hours_exported_q3_callback)(void);
  uint32_t (*total_v_ar_hours_exported_q3_phase_a_callback)(void);
  uint32_t (*total_v_ar_hours_exported_q3_phase_b_callback)(void);
  uint32_t (*total_v_ar_hours_exported_q3_phase_c_callback)(void);
  uint32_t (*total_v_ar_hours_exported_q4_callback)(void);
  uint32_t (*total_v_ar_hours_exported_q4_imported_phase_a_callback)(void);
  uint32_t (*total_v_ar_hours_exported_q4_imported_phase_b_callback)(void);
  uint32_t (*total_v_ar_hours_exported_q4_imported_phase_c_callback)(void);
  uint16_t (*tot_v_arh_sf_callback)(void);
  uint32_t (*events_callback)(void);
} Model204CallbackAdapter;

typedef struct Model211CallbackAdapter {
  float (*amps_callback)(void);
  float (*amps_phase_a_callback)(void);
  float (*amps_phase_b_callback)(void);
  float (*amps_phase_c_callback)(void);
  float (*voltage_ln_callback)(void);
  float (*phase_voltage_an_callback)(void);
  float (*phase_voltage_bn_callback)(void);
  float (*phase_voltage_cn_callback)(void);
  float (*voltage_ll_callback)(void);
  float (*phase_voltage_ab_callback)(void);
  float (*phase_voltage_bc_callback)(void);
  float (*phase_voltage_ca_callback)(void);
  float (*hz_callback)(void);
  float (*watts_callback)(void);
  float (*watts_phase_a_callback)(void);
  float (*watts_phase_b_callback)(void);
  float (*watts_phase_c_callback)(void);
  float (*va_callback)(void);
  float (*va_phase_a_callback)(void);
  float (*va_phase_b_callback)(void);
  float (*va_phase_c_callback)(void);
  float (*var_callback)(void);
  float (*var_phase_a_callback)(void);
  float (*var_phase_b_callback)(void);
  float (*var_phase_c_callback)(void);
  float (*pf_callback)(void);
  float (*pf_phase_a_callback)(void);
  float (*pf_phase_b_callback)(void);
  float (*pf_phase_c_callback)(void);
  float (*total_watt_hours_exported_callback)(void);
  float (*total_watt_hours_exported_phase_a_callback)(void);
  float (*total_watt_hours_exported_phase_b_callback)(void);
  float (*total_watt_hours_exported_phase_c_callback)(void);
  float (*total_watt_hours_imported_callback)(void);
  float (*total_watt_hours_imported_phase_a_callback)(void);
  float (*total_watt_hours_imported_phase_b_callback)(void);
  float (*total_watt_hours_imported_phase_c_callback)(void);
  float (*total_va_hours_exported_callback)(void);
  float (*total_va_hours_exported_phase_a_callback)(void);
  float (*total_va_hours_exported_phase_b_callback)(void);
  float (*total_va_hours_exported_phase_c_callback)(void);
  float (*total_va_hours_imported_callback)(void);
  float (*total_va_hours_imported_phase_a_callback)(void);
  float (*total_va_hours_imported_phase_b_callback)(void);
  float (*total_va_hours_imported_phase_c_callback)(void);
  float (*total_var_hours_imported_q1_callback)(void);
  float (*total_v_ar_hours_imported_q1_phase_a_callback)(void);
  float (*total_v_ar_hours_imported_q1_phase_b_callback)(void);
  float (*total_v_ar_hours_imported_q1_phase_c_callback)(void);
  float (*total_v_ar_hours_imported_q2_callback)(void);
  float (*total_v_ar_hours_imported_q2_phase_a_callback)(void);
  float (*total_v_ar_hours_imported_q2_phase_b_callback)(void);
  float (*total_v_ar_hours_imported_q2_phase_c_callback)(void);
  float (*total_v_ar_hours_exported_q3_callback)(void);
  float (*total_v_ar_hours_exported_q3_phase_a_callback)(void);
  float (*total_v_ar_hours_exported_q3_phase_b_callback)(void);
  float (*total_v_ar_hours_exported_q3_phase_c_callback)(void);
  float (*total_v_ar_hours_exported_q4_callback)(void);
  float (*total_v_ar_hours_exported_q4_imported_phase_a_callback)(void);
  float (*total_v_ar_hours_exported_q4_imported_phase_b_callback)(void);
  float (*total_v_ar_hours_exported_q4_imported_phase_c_callback)(void);
  uint32_t (*events_callback)(void);
} Model211CallbackAdapter;

typedef struct Model212CallbackAdapter {
  float (*amps_callback)(void);
  float (*amps_phase_a_callback)(void);
  float (*amps_phase_b_callback)(void);
  float (*amps_phase_c_callback)(void);
  float (*voltage_ln_callback)(void);
  float (*phase_voltage_an_callback)(void);
  float (*phase_voltage_bn_callback)(void);
  float (*phase_voltage_cn_callback)(void);
  float (*voltage_ll_callback)(void);
  float (*phase_voltage_ab_callback)(void);
  float (*phase_voltage_bc_callback)(void);
  float (*phase_voltage_ca_callback)(void);
  float (*hz_callback)(void);
  float (*watts_callback)(void);
  float (*watts_phase_a_callback)(void);
  float (*watts_phase_b_callback)(void);
  float (*watts_phase_c_callback)(void);
  float (*va_callback)(void);
  float (*va_phase_a_callback)(void);
  float (*va_phase_b_callback)(void);
  float (*va_phase_c_callback)(void);
  float (*var_callback)(void);
  float (*var_phase_a_callback)(void);
  float (*var_phase_b_callback)(void);
  float (*var_phase_c_callback)(void);
  float (*pf_callback)(void);
  float (*pf_phase_a_callback)(void);
  float (*pf_phase_b_callback)(void);
  float (*pf_phase_c_callback)(void);
  float (*total_watt_hours_exported_callback)(void);
  float (*total_watt_hours_exported_phase_a_callback)(void);
  float (*total_watt_hours_exported_phase_b_callback)(void);
  float (*total_watt_hours_exported_phase_c_callback)(void);
  float (*total_watt_hours_imported_callback)(void);
  float (*total_watt_hours_imported_phase_a_callback)(void);
  float (*total_watt_hours_imported_phase_b_callback)(void);
  float (*total_watt_hours_imported_phase_c_callback)(void);
  float (*total_va_hours_exported_callback)(void);
  float (*total_va_hours_exported_phase_a_callback)(void);
  float (*total_va_hours_exported_phase_b_callback)(void);
  float (*total_va_hours_exported_phase_c_callback)(void);
  float (*total_va_hours_imported_callback)(void);
  float (*total_va_hours_imported_phase_a_callback)(void);
  float (*total_va_hours_imported_phase_b_callback)(void);
  float (*total_va_hours_imported_phase_c_callback)(void);
  float (*total_var_hours_imported_q1_callback)(void);
  float (*total_v_ar_hours_imported_q1_phase_a_callback)(void);
  float (*total_v_ar_hours_imported_q1_phase_b_callback)(void);
  float (*total_v_ar_hours_imported_q1_phase_c_callback)(void);
  float (*total_v_ar_hours_imported_q2_callback)(void);
  float (*total_v_ar_hours_imported_q2_phase_a_callback)(void);
  float (*total_v_ar_hours_imported_q2_phase_b_callback)(void);
  float (*total_v_ar_hours_imported_q2_phase_c_callback)(void);
  float (*total_v_ar_hours_exported_q3_callback)(void);
  float (*total_v_ar_hours_exported_q3_phase_a_callback)(void);
  float (*total_v_ar_hours_exported_q3_phase_b_callback)(void);
  float (*total_v_ar_hours_exported_q3_phase_c_callback)(void);
  float (*total_v_ar_hours_exported_q4_callback)(void);
  float (*total_v_ar_hours_exported_q4_imported_phase_a_callback)(void);
  float (*total_v_ar_hours_exported_q4_imported_phase_b_callback)(void);
  float (*total_v_ar_hours_exported_q4_imported_phase_c_callback)(void);
  uint32_t (*events_callback)(void);
} Model212CallbackAdapter;

typedef struct Model213CallbackAdapter {
  float (*amps_callback)(void);
  float (*amps_phase_a_callback)(void);
  float (*amps_phase_b_callback)(void);
  float (*amps_phase_c_callback)(void);
  float (*voltage_ln_callback)(void);
  float (*phase_voltage_an_callback)(void);
  float (*phase_voltage_bn_callback)(void);
  float (*phase_voltage_cn_callback)(void);
  float (*voltage_ll_callback)(void);
  float (*phase_voltage_ab_callback)(void);
  float (*phase_voltage_bc_callback)(void);
  float (*phase_voltage_ca_callback)(void);
  float (*hz_callback)(void);
  float (*watts_callback)(void);
  float (*watts_phase_a_callback)(void);
  float (*watts_phase_b_callback)(void);
  float (*watts_phase_c_callback)(void);
  float (*va_callback)(void);
  float (*va_phase_a_callback)(void);
  float (*va_phase_b_callback)(void);
  float (*va_phase_c_callback)(void);
  float (*var_callback)(void);
  float (*var_phase_a_callback)(void);
  float (*var_phase_b_callback)(void);
  float (*var_phase_c_callback)(void);
  float (*pf_callback)(void);
  float (*pf_phase_a_callback)(void);
  float (*pf_phase_b_callback)(void);
  float (*pf_phase_c_callback)(void);
  float (*total_watt_hours_exported_callback)(void);
  float (*total_watt_hours_exported_phase_a_callback)(void);
  float (*total_watt_hours_exported_phase_b_callback)(void);
  float (*total_watt_hours_exported_phase_c_callback)(void);
  float (*total_watt_hours_imported_callback)(void);
  float (*total_watt_hours_imported_phase_a_callback)(void);
  float (*total_watt_hours_imported_phase_b_callback)(void);
  float (*total_watt_hours_imported_phase_c_callback)(void);
  float (*total_va_hours_exported_callback)(void);
  float (*total_va_hours_exported_phase_a_callback)(void);
  float (*total_va_hours_exported_phase_b_callback)(void);
  float (*total_va_hours_exported_phase_c_callback)(void);
  float (*total_va_hours_imported_callback)(void);
  float (*total_va_hours_imported_phase_a_callback)(void);
  float (*total_va_hours_imported_phase_b_callback)(void);
  float (*total_va_hours_imported_phase_c_callback)(void);
  float (*total_var_hours_imported_q1_callback)(void);
  float (*total_v_ar_hours_imported_q1_phase_a_callback)(void);
  float (*total_v_ar_hours_imported_q1_phase_b_callback)(void);
  float (*total_v_ar_hours_imported_q1_phase_c_callback)(void);
  float (*total_v_ar_hours_imported_q2_callback)(void);
  float (*total_v_ar_hours_imported_q2_phase_a_callback)(void);
  float (*total_v_ar_hours_imported_q2_phase_b_callback)(void);
  float (*total_v_ar_hours_imported_q2_phase_c_callback)(void);
  float (*total_v_ar_hours_exported_q3_callback)(void);
  float (*total_v_ar_hours_exported_q3_phase_a_callback)(void);
  float (*total_v_ar_hours_exported_q3_phase_b_callback)(void);
  float (*total_v_ar_hours_exported_q3_phase_c_callback)(void);
  float (*total_v_ar_hours_exported_q4_callback)(void);
  float (*total_v_ar_hours_exported_q4_imported_phase_a_callback)(void);
  float (*total_v_ar_hours_exported_q4_imported_phase_b_callback)(void);
  float (*total_v_ar_hours_exported_q4_imported_phase_c_callback)(void);
  uint32_t (*events_callback)(void);
} Model213CallbackAdapter;

typedef struct Model214CallbackAdapter {
  float (*amps_callback)(void);
  float (*amps_phase_a_callback)(void);
  float (*amps_phase_b_callback)(void);
  float (*amps_phase_c_callback)(void);
  float (*voltage_ln_callback)(void);
  float (*phase_voltage_an_callback)(void);
  float (*phase_voltage_bn_callback)(void);
  float (*phase_voltage_cn_callback)(void);
  float (*voltage_ll_callback)(void);
  float (*phase_voltage_ab_callback)(void);
  float (*phase_voltage_bc_callback)(void);
  float (*phase_voltage_ca_callback)(void);
  float (*hz_callback)(void);
  float (*watts_callback)(void);
  float (*watts_phase_a_callback)(void);
  float (*watts_phase_b_callback)(void);
  float (*watts_phase_c_callback)(void);
  float (*va_callback)(void);
  float (*va_phase_a_callback)(void);
  float (*va_phase_b_callback)(void);
  float (*va_phase_c_callback)(void);
  float (*var_callback)(void);
  float (*var_phase_a_callback)(void);
  float (*var_phase_b_callback)(void);
  float (*var_phase_c_callback)(void);
  float (*pf_callback)(void);
  float (*pf_phase_a_callback)(void);
  float (*pf_phase_b_callback)(void);
  float (*pf_phase_c_callback)(void);
  float (*total_watt_hours_exported_callback)(void);
  float (*total_watt_hours_exported_phase_a_callback)(void);
  float (*total_watt_hours_exported_phase_b_callback)(void);
  float (*total_watt_hours_exported_phase_c_callback)(void);
  float (*total_watt_hours_imported_callback)(void);
  float (*total_watt_hours_imported_phase_a_callback)(void);
  float (*total_watt_hours_imported_phase_b_callback)(void);
  float (*total_watt_hours_imported_phase_c_callback)(void);
  float (*total_va_hours_exported_callback)(void);
  float (*total_va_hours_exported_phase_a_callback)(void);
  float (*total_va_hours_exported_phase_b_callback)(void);
  float (*total_va_hours_exported_phase_c_callback)(void);
  float (*total_va_hours_imported_callback)(void);
  float (*total_va_hours_imported_phase_a_callback)(void);
  float (*total_va_hours_imported_phase_b_callback)(void);
  float (*total_va_hours_imported_phase_c_callback)(void);
  float (*total_var_hours_imported_q1_callback)(void);
  float (*total_v_ar_hours_imported_q1_phase_a_callback)(void);
  float (*total_v_ar_hours_imported_q1_phase_b_callback)(void);
  float (*total_v_ar_hours_imported_q1_phase_c_callback)(void);
  float (*total_v_ar_hours_imported_q2_callback)(void);
  float (*total_v_ar_hours_imported_q2_phase_a_callback)(void);
  float (*total_v_ar_hours_imported_q2_phase_b_callback)(void);
  float (*total_v_ar_hours_imported_q2_phase_c_callback)(void);
  float (*total_v_ar_hours_exported_q3_callback)(void);
  float (*total_v_ar_hours_exported_q3_phase_a_callback)(void);
  float (*total_v_ar_hours_exported_q3_phase_b_callback)(void);
  float (*total_v_ar_hours_exported_q3_phase_c_callback)(void);
  float (*total_v_ar_hours_exported_q4_callback)(void);
  float (*total_v_ar_hours_exported_q4_imported_phase_a_callback)(void);
  float (*total_v_ar_hours_exported_q4_imported_phase_b_callback)(void);
  float (*total_v_ar_hours_exported_q4_imported_phase_c_callback)(void);
  uint32_t (*events_callback)(void);
} Model214CallbackAdapter;

typedef struct Model220CallbackAdapter {
  int16_t (*amps_callback)(void);
  uint16_t (*a_sf_callback)(void);
  int16_t (*voltage_callback)(void);
  uint16_t (*v_sf_callback)(void);
  int16_t (*hz_callback)(void);
  uint16_t (*hz_sf_callback)(void);
  int16_t (*watts_callback)(void);
  uint16_t (*w_sf_callback)(void);
  int16_t (*va_callback)(void);
  uint16_t (*va_sf_callback)(void);
  int16_t (*var_callback)(void);
  uint16_t (*var_sf_callback)(void);
  int16_t (*pf_callback)(void);
  uint16_t (*pf_sf_callback)(void);
  uint32_t (*total_watt_hours_exported_callback)(void);
  uint32_t (*total_watt_hours_imported_callback)(void);
  uint16_t (*tot_wh_sf_callback)(void);
  uint32_t (*total_va_hours_exported_callback)(void);
  uint32_t (*total_va_hours_imported_callback)(void);
  uint16_t (*tot_v_ah_sf_callback)(void);
  uint32_t (*total_var_hours_imported_q1_callback)(void);
  uint32_t (*total_v_ar_hours_imported_q2_callback)(void);
  uint32_t (*total_v_ar_hours_exported_q3_callback)(void);
  uint32_t (*total_v_ar_hours_exported_q4_callback)(void);
  uint16_t (*tot_v_arh_sf_callback)(void);
  uint32_t (*events_callback)(void);
  uint32_t (*timestamp_callback)(void);
  uint16_t (*milliseconds_callback)(void);
  uint16_t (*sequence_callback)(void);
  Alg (*algorithm_callback)(void);
  uint16_t (*n_callback)(void);
} Model220CallbackAdapter;

typedef struct Model3CallbackAdapter {
  uint16_t (*x_callback)(void);
  void (*set_x_callback)(uint16_t);
  uint16_t (*offset1_callback)(void);
  void (*set_offset1_callback)(uint16_t);
  uint16_t (*off2_callback)(void);
  void (*set_off2_callback)(uint16_t);
  uint16_t (*off3_callback)(void);
  void (*set_off3_callback)(uint16_t);
  uint16_t (*off4_callback)(void);
  void (*set_off4_callback)(uint16_t);
  uint16_t (*off5_callback)(void);
  void (*set_off5_callback)(uint16_t);
  uint16_t (*off6_callback)(void);
  void (*set_off6_callback)(uint16_t);
  uint16_t (*off7_callback)(void);
  void (*set_off7_callback)(uint16_t);
  uint16_t (*off8_callback)(void);
  void (*set_off8_callback)(uint16_t);
  uint16_t (*off9_callback)(void);
  void (*set_off9_callback)(uint16_t);
  uint16_t (*off10_callback)(void);
  void (*set_off10_callback)(uint16_t);
  uint16_t (*off11_callback)(void);
  void (*set_off11_callback)(uint16_t);
  uint16_t (*off12_callback)(void);
  void (*set_off12_callback)(uint16_t);
  uint16_t (*off13_callback)(void);
  void (*set_off13_callback)(uint16_t);
  uint16_t (*off14_callback)(void);
  void (*set_off14_callback)(uint16_t);
  uint16_t (*off15_callback)(void);
  void (*set_off15_callback)(uint16_t);
  uint16_t (*off16_callback)(void);
  void (*set_off16_callback)(uint16_t);
  uint16_t (*off17_callback)(void);
  void (*set_off17_callback)(uint16_t);
  uint16_t (*off18_callback)(void);
  void (*set_off18_callback)(uint16_t);
  uint16_t (*off19_callback)(void);
  void (*set_off19_callback)(uint16_t);
  uint16_t (*off20_callback)(void);
  void (*set_off20_callback)(uint16_t);
  uint16_t (*off21_callback)(void);
  void (*set_off21_callback)(uint16_t);
  uint16_t (*off22_callback)(void);
  void (*set_off22_callback)(uint16_t);
  uint16_t (*off23_callback)(void);
  void (*set_off23_callback)(uint16_t);
  uint16_t (*off24_callback)(void);
  void (*set_off24_callback)(uint16_t);
  uint16_t (*off25_callback)(void);
  void (*set_off25_callback)(uint16_t);
  uint16_t (*off26_callback)(void);
  void (*set_off26_callback)(uint16_t);
  uint16_t (*off27_callback)(void);
  void (*set_off27_callback)(uint16_t);
  uint16_t (*off28_callback)(void);
  void (*set_off28_callback)(uint16_t);
  uint16_t (*off29_callback)(void);
  void (*set_off29_callback)(uint16_t);
  uint16_t (*off30_callback)(void);
  void (*set_off30_callback)(uint16_t);
  uint16_t (*off31_callback)(void);
  void (*set_off31_callback)(uint16_t);
  uint16_t (*off32_callback)(void);
  void (*set_off32_callback)(uint16_t);
  uint16_t (*off33_callback)(void);
  void (*set_off33_callback)(uint16_t);
  uint16_t (*off34_callback)(void);
  void (*set_off34_callback)(uint16_t);
  uint16_t (*off35_callback)(void);
  void (*set_off35_callback)(uint16_t);
  uint16_t (*off36_callback)(void);
  void (*set_off36_callback)(uint16_t);
  uint16_t (*off37_callback)(void);
  void (*set_off37_callback)(uint16_t);
  uint16_t (*off38_callback)(void);
  void (*set_off38_callback)(uint16_t);
  uint16_t (*off39_callback)(void);
  void (*set_off39_callback)(uint16_t);
  uint16_t (*off40_callback)(void);
  void (*set_off40_callback)(uint16_t);
  uint16_t (*off41_callback)(void);
  void (*set_off41_callback)(uint16_t);
  uint16_t (*off42_callback)(void);
  void (*set_off42_callback)(uint16_t);
  uint16_t (*off43_callback)(void);
  void (*set_off43_callback)(uint16_t);
  uint16_t (*off44_callback)(void);
  void (*set_off44_callback)(uint16_t);
  uint16_t (*off45_callback)(void);
  void (*set_off45_callback)(uint16_t);
  uint16_t (*off46_callback)(void);
  void (*set_off46_callback)(uint16_t);
  uint16_t (*off47_callback)(void);
  void (*set_off47_callback)(uint16_t);
  uint16_t (*off48_callback)(void);
  void (*set_off48_callback)(uint16_t);
  uint16_t (*off49_callback)(void);
  void (*set_off49_callback)(uint16_t);
  uint16_t (*off50_callback)(void);
  void (*set_off50_callback)(uint16_t);
  uint32_t (*timestamp_callback)(void);
  void (*set_timestamp_callback)(uint32_t);
  uint16_t (*milliseconds_callback)(void);
  void (*set_milliseconds_callback)(uint16_t);
  uint16_t (*sequence_callback)(void);
  void (*set_sequence_callback)(uint16_t);
  uint16_t (*role_callback)(void);
  void (*set_role_callback)(uint16_t);
  Alg (*algorithm_callback)(void);
  uint16_t (*n_callback)(void);
} Model3CallbackAdapter;

typedef struct Model305CallbackAdapter {
  const char *(*tm_callback)(void);
  const char *(*date_callback)(void);
  const char *(*location_callback)(void);
  int32_t (*lat_callback)(void);
  int32_t (*long_callback)(void);
  int32_t (*altitude_callback)(void);
} Model305CallbackAdapter;

typedef struct Model306CallbackAdapter {
  uint16_t (*ghi_callback)(void);
  uint16_t (*amps_callback)(void);
  uint16_t (*voltage_callback)(void);
  uint16_t (*temperature_callback)(void);
} Model306CallbackAdapter;

typedef struct Model307CallbackAdapter {
  int16_t (*ambient_temperature_callback)(void);
  int16_t (*relative_humidity_callback)(void);
  int16_t (*barometric_pressure_callback)(void);
  int16_t (*wind_speed_callback)(void);
  int16_t (*wind_direction_callback)(void);
  int16_t (*rainfall_callback)(void);
  int16_t (*snow_depth_callback)(void);
  int16_t (*precipitation_type_callback)(void);
  int16_t (*electric_field_callback)(void);
  int16_t (*surface_wetness_callback)(void);
  int16_t (*soil_wetness_callback)(void);
} Model307CallbackAdapter;

typedef struct Model308CallbackAdapter {
  uint16_t (*ghi_callback)(void);
  int16_t (*temp_callback)(void);
  int16_t (*ambient_temperature_callback)(void);
  uint16_t (*wind_speed_callback)(void);
} Model308CallbackAdapter;

typedef struct Model4CallbackAdapter {
  uint16_t (*request_sequence_callback)(void);
  Sts (*status_callback)(void);
  uint16_t (*x_callback)(void);
  uint16_t (*value1_callback)(void);
  uint16_t (*val2_callback)(void);
  uint16_t (*val3_callback)(void);
  uint16_t (*val4_callback)(void);
  uint16_t (*val5_callback)(void);
  uint16_t (*val6_callback)(void);
  uint16_t (*val7_callback)(void);
  uint16_t (*val8_callback)(void);
  uint16_t (*val9_callback)(void);
  uint16_t (*val10_callback)(void);
  uint16_t (*val11_callback)(void);
  uint16_t (*val12_callback)(void);
  uint16_t (*val13_callback)(void);
  uint16_t (*val14_callback)(void);
  uint16_t (*val15_callback)(void);
  uint16_t (*val16_callback)(void);
  uint16_t (*val17_callback)(void);
  uint16_t (*val18_callback)(void);
  uint16_t (*val19_callback)(void);
  uint16_t (*val20_callback)(void);
  uint16_t (*val21_callback)(void);
  uint16_t (*val22_callback)(void);
  uint16_t (*val23_callback)(void);
  uint16_t (*val24_callback)(void);
  uint16_t (*val25_callback)(void);
  uint16_t (*val26_callback)(void);
  uint16_t (*val27_callback)(void);
  uint16_t (*val28_callback)(void);
  uint16_t (*val29_callback)(void);
  uint16_t (*val30_callback)(void);
  uint16_t (*val31_callback)(void);
  uint16_t (*val32_callback)(void);
  uint16_t (*val33_callback)(void);
  uint16_t (*val34_callback)(void);
  uint16_t (*val35_callback)(void);
  uint16_t (*val36_callback)(void);
  uint16_t (*val37_callback)(void);
  uint16_t (*val38_callback)(void);
  uint16_t (*val39_callback)(void);
  uint16_t (*val40_callback)(void);
  uint16_t (*val41_callback)(void);
  uint16_t (*val42_callback)(void);
  uint16_t (*val43_callback)(void);
  uint16_t (*val44_callback)(void);
  uint16_t (*val45_callback)(void);
  uint16_t (*val46_callback)(void);
  uint16_t (*val47_callback)(void);
  uint16_t (*val48_callback)(void);
  uint16_t (*val49_callback)(void);
  uint16_t (*val50_callback)(void);
  uint32_t (*timestamp_callback)(void);
  uint16_t (*milliseconds_callback)(void);
  uint16_t (*sequence_callback)(void);
  Alm (*alarm_callback)(void);
  Alg (*algorithm_callback)(void);
  uint16_t (*n_callback)(void);
} Model4CallbackAdapter;

typedef struct Model401CallbackAdapter {
  uint16_t (*dca_sf_callback)(void);
  uint16_t (*dc_ahr_sf_callback)(void);
  uint16_t (*dcv_sf_callback)(void);
  uint16_t (*rating_callback)(void);
  uint16_t (*n_callback)(void);
  uint32_t (*event_callback)(void);
  uint32_t (*vendor_event_callback)(void);
  int16_t (*amps_callback)(void);
  uint32_t (*amp_hours_callback)(void);
  uint16_t (*voltage_callback)(void);
  int16_t (*temp_callback)(void);
} Model401CallbackAdapter;

typedef struct Model402CallbackAdapter {
  uint16_t (*dca_sf_callback)(void);
  uint16_t (*dc_ahr_sf_callback)(void);
  uint16_t (*dcv_sf_callback)(void);
  uint16_t (*dcw_sf_callback)(void);
  uint16_t (*dc_wh_sf_callback)(void);
  uint16_t (*rating_callback)(void);
  uint16_t (*n_callback)(void);
  uint32_t (*event_callback)(void);
  uint32_t (*vendor_event_callback)(void);
  int16_t (*amps_callback)(void);
  uint32_t (*amp_hours_callback)(void);
  uint16_t (*voltage_callback)(void);
  int16_t (*temp_callback)(void);
  int16_t (*watts_callback)(void);
  uint16_t (*pr_callback)(void);
  uint32_t (*watt_hours_callback)(void);
} Model402CallbackAdapter;

typedef struct Model403CallbackAdapter {
  uint16_t (*dca_sf_callback)(void);
  uint16_t (*dc_ahr_sf_callback)(void);
  uint16_t (*dcv_sf_callback)(void);
  uint16_t (*rating_callback)(void);
  uint16_t (*n_callback)(void);
  uint32_t (*event_callback)(void);
  uint32_t (*vendor_event_callback)(void);
  int16_t (*amps_callback)(void);
  uint32_t (*amp_hours_callback)(void);
  int16_t (*voltage_callback)(void);
  int16_t (*temp_callback)(void);
  uint16_t (*in_dca_sf_callback)(void);
  uint16_t (*in_dc_ahr_sf_callback)(void);
} Model403CallbackAdapter;

typedef struct Model404CallbackAdapter {
  uint16_t (*dca_sf_callback)(void);
  uint16_t (*dc_ahr_sf_callback)(void);
  uint16_t (*dcv_sf_callback)(void);
  uint16_t (*dcw_sf_callback)(void);
  uint16_t (*dc_wh_sf_callback)(void);
  uint16_t (*rating_callback)(void);
  uint16_t (*n_callback)(void);
  uint32_t (*event_callback)(void);
  uint32_t (*vendor_event_callback)(void);
  int16_t (*amps_callback)(void);
  uint32_t (*amp_hours_callback)(void);
  int16_t (*voltage_callback)(void);
  int16_t (*temp_callback)(void);
  int16_t (*watts_callback)(void);
  int16_t (*pr_callback)(void);
  uint32_t (*watt_hours_callback)(void);
  uint16_t (*in_dca_sf_callback)(void);
  uint16_t (*in_dc_ahr_sf_callback)(void);
  uint16_t (*in_dcv_sf_callback)(void);
  uint16_t (*in_dcw_sf_callback)(void);
  uint16_t (*in_dc_wh_sf_callback)(void);
} Model404CallbackAdapter;

typedef struct Model5CallbackAdapter {
  uint16_t (*x_callback)(void);
  void (*set_x_callback)(uint16_t);
  uint16_t (*offset1_callback)(void);
  void (*set_offset1_callback)(uint16_t);
  uint16_t (*value1_callback)(void);
  void (*set_value1_callback)(uint16_t);
  uint16_t (*off2_callback)(void);
  void (*set_off2_callback)(uint16_t);
  uint16_t (*val2_callback)(void);
  void (*set_val2_callback)(uint16_t);
  uint16_t (*off3_callback)(void);
  void (*set_off3_callback)(uint16_t);
  uint16_t (*val3_callback)(void);
  void (*set_val3_callback)(uint16_t);
  uint16_t (*off4_callback)(void);
  void (*set_off4_callback)(uint16_t);
  uint16_t (*val4_callback)(void);
  void (*set_val4_callback)(uint16_t);
  uint16_t (*off5_callback)(void);
  void (*set_off5_callback)(uint16_t);
  uint16_t (*val5_callback)(void);
  void (*set_val5_callback)(uint16_t);
  uint16_t (*off6_callback)(void);
  void (*set_off6_callback)(uint16_t);
  uint16_t (*val6_callback)(void);
  void (*set_val6_callback)(uint16_t);
  uint16_t (*off7_callback)(void);
  void (*set_off7_callback)(uint16_t);
  uint16_t (*val7_callback)(void);
  void (*set_val7_callback)(uint16_t);
  uint16_t (*off8_callback)(void);
  void (*set_off8_callback)(uint16_t);
  uint16_t (*val8_callback)(void);
  void (*set_val8_callback)(uint16_t);
  uint16_t (*off9_callback)(void);
  void (*set_off9_callback)(uint16_t);
  uint16_t (*val9_callback)(void);
  void (*set_val9_callback)(uint16_t);
  uint16_t (*off10_callback)(void);
  void (*set_off10_callback)(uint16_t);
  uint16_t (*val10_callback)(void);
  void (*set_val10_callback)(uint16_t);
  uint16_t (*off11_callback)(void);
  void (*set_off11_callback)(uint16_t);
  uint16_t (*val11_callback)(void);
  void (*set_val11_callback)(uint16_t);
  uint16_t (*off12_callback)(void);
  void (*set_off12_callback)(uint16_t);
  uint16_t (*val12_callback)(void);
  void (*set_val12_callback)(uint16_t);
  uint16_t (*off13_callback)(void);
  void (*set_off13_callback)(uint16_t);
  uint16_t (*val13_callback)(void);
  void (*set_val13_callback)(uint16_t);
  uint16_t (*off14_callback)(void);
  void (*set_off14_callback)(uint16_t);
  uint16_t (*val14_callback)(void);
  void (*set_val14_callback)(uint16_t);
  uint16_t (*off15_callback)(void);
  void (*set_off15_callback)(uint16_t);
  uint16_t (*val15_callback)(void);
  void (*set_val15_callback)(uint16_t);
  uint16_t (*off16_callback)(void);
  void (*set_off16_callback)(uint16_t);
  uint16_t (*val16_callback)(void);
  void (*set_val16_callback)(uint16_t);
  uint16_t (*off17_callback)(void);
  void (*set_off17_callback)(uint16_t);
  uint16_t (*val17_callback)(void);
  void (*set_val17_callback)(uint16_t);
  uint16_t (*off18_callback)(void);
  void (*set_off18_callback)(uint16_t);
  uint16_t (*val18_callback)(void);
  void (*set_val18_callback)(uint16_t);
  uint16_t (*off19_callback)(void);
  void (*set_off19_callback)(uint16_t);
  uint16_t (*val19_callback)(void);
  void (*set_val19_callback)(uint16_t);
  uint16_t (*off20_callback)(void);
  void (*set_off20_callback)(uint16_t);
  uint16_t (*val20_callback)(void);
  void (*set_val20_callback)(uint16_t);
  uint16_t (*off21_callback)(void);
  void (*set_off21_callback)(uint16_t);
  uint16_t (*val21_callback)(void);
  void (*set_val21_callback)(uint16_t);
  uint16_t (*off22_callback)(void);
  void (*set_off22_callback)(uint16_t);
  uint16_t (*val22_callback)(void);
  void (*set_val22_callback)(uint16_t);
  uint16_t (*off23_callback)(void);
  void (*set_off23_callback)(uint16_t);
  uint16_t (*val23_callback)(void);
  void (*set_val23_callback)(uint16_t);
  uint16_t (*off24_callback)(void);
  void (*set_off24_callback)(uint16_t);
  uint16_t (*val24_callback)(void);
  void (*set_val24_callback)(uint16_t);
  uint16_t (*off25_callback)(void);
  void (*set_off25_callback)(uint16_t);
  uint16_t (*val25_callback)(void);
  void (*set_val25_callback)(uint16_t);
  uint16_t (*off26_callback)(void);
  void (*set_off26_callback)(uint16_t);
  uint16_t (*val26_callback)(void);
  void (*set_val26_callback)(uint16_t);
  uint16_t (*off27_callback)(void);
  void (*set_off27_callback)(uint16_t);
  uint16_t (*val27_callback)(void);
  void (*set_val27_callback)(uint16_t);
  uint16_t (*off28_callback)(void);
  void (*set_off28_callback)(uint16_t);
  uint16_t (*val28_callback)(void);
  void (*set_val28_callback)(uint16_t);
  uint16_t (*off29_callback)(void);
  void (*set_off29_callback)(uint16_t);
  uint16_t (*val29_callback)(void);
  void (*set_val29_callback)(uint16_t);
  uint16_t (*off30_callback)(void);
  void (*set_off30_callback)(uint16_t);
  uint16_t (*val30_callback)(void);
  void (*set_val30_callback)(uint16_t);
  uint16_t (*off31_callback)(void);
  void (*set_off31_callback)(uint16_t);
  uint16_t (*val31_callback)(void);
  void (*set_val31_callback)(uint16_t);
  uint16_t (*off32_callback)(void);
  void (*set_off32_callback)(uint16_t);
  uint16_t (*val32_callback)(void);
  void (*set_val32_callback)(uint16_t);
  uint16_t (*off33_callback)(void);
  void (*set_off33_callback)(uint16_t);
  uint16_t (*val33_callback)(void);
  void (*set_val33_callback)(uint16_t);
  uint16_t (*off34_callback)(void);
  void (*set_off34_callback)(uint16_t);
  uint16_t (*val34_callback)(void);
  void (*set_val34_callback)(uint16_t);
  uint16_t (*off35_callback)(void);
  void (*set_off35_callback)(uint16_t);
  uint16_t (*val35_callback)(void);
  void (*set_val35_callback)(uint16_t);
  uint16_t (*off36_callback)(void);
  void (*set_off36_callback)(uint16_t);
  uint16_t (*val36_callback)(void);
  void (*set_val36_callback)(uint16_t);
  uint16_t (*off37_callback)(void);
  void (*set_off37_callback)(uint16_t);
  uint16_t (*val37_callback)(void);
  void (*set_val37_callback)(uint16_t);
  uint16_t (*off38_callback)(void);
  void (*set_off38_callback)(uint16_t);
  uint16_t (*val38_callback)(void);
  void (*set_val38_callback)(uint16_t);
  uint16_t (*off39_callback)(void);
  void (*set_off39_callback)(uint16_t);
  uint16_t (*val39_callback)(void);
  void (*set_val39_callback)(uint16_t);
  uint16_t (*off40_callback)(void);
  void (*set_off40_callback)(uint16_t);
  uint16_t (*val40_callback)(void);
  void (*set_val40_callback)(uint16_t);
  uint32_t (*timestamp_callback)(void);
  void (*set_timestamp_callback)(uint32_t);
  uint16_t (*milliseconds_callback)(void);
  void (*set_milliseconds_callback)(uint16_t);
  uint16_t (*sequence_callback)(void);
  void (*set_sequence_callback)(uint16_t);
  uint16_t (*role_callback)(void);
  void (*set_role_callback)(uint16_t);
  Alg (*algorithm_callback)(void);
  void (*set_algorithm_callback)(Alg);
  uint16_t (*n_callback)(void);
  void (*set_n_callback)(uint16_t);
} Model5CallbackAdapter;

typedef struct Model501CallbackAdapter {
  Stat (*status_callback)(void);
  uint16_t (*vendor_status_callback)(void);
  uint32_t (*events_callback)(void);
  uint32_t (*vendor_module_event_flags_callback)(void);
  uint16_t (*control_callback)(void);
  void (*set_control_callback)(uint16_t);
  uint32_t (*vendor_control_callback)(void);
  void (*set_vendor_control_callback)(uint32_t);
  int32_t (*control_value_callback)(void);
  void (*set_control_value_callback)(int32_t);
  uint32_t (*timestamp_callback)(void);
  float (*output_current_callback)(void);
  float (*output_voltage_callback)(void);
  float (*output_energy_callback)(void);
  float (*output_power_callback)(void);
  float (*temp_callback)(void);
  float (*input_current_callback)(void);
  float (*input_voltage_callback)(void);
  float (*input_energy_callback)(void);
  float (*input_power_callback)(void);
} Model501CallbackAdapter;

typedef struct Model502CallbackAdapter {
  uint16_t (*a_sf_callback)(void);
  uint16_t (*v_sf_callback)(void);
  uint16_t (*w_sf_callback)(void);
  uint16_t (*wh_sf_callback)(void);
  Stat (*status_callback)(void);
  uint16_t (*vendor_status_callback)(void);
  uint32_t (*events_callback)(void);
  uint32_t (*vendor_module_event_flags_callback)(void);
  uint16_t (*control_callback)(void);
  void (*set_control_callback)(uint16_t);
  uint32_t (*vendor_control_callback)(void);
  void (*set_vendor_control_callback)(uint32_t);
  int32_t (*control_value_callback)(void);
  void (*set_control_value_callback)(int32_t);
  uint32_t (*timestamp_callback)(void);
  int16_t (*output_current_callback)(void);
  int16_t (*output_voltage_callback)(void);
  uint32_t (*output_energy_callback)(void);
  int16_t (*output_power_callback)(void);
  int16_t (*temp_callback)(void);
  int16_t (*input_current_callback)(void);
  int16_t (*input_voltage_callback)(void);
  uint32_t (*input_energy_callback)(void);
  int16_t (*input_power_callback)(void);
} Model502CallbackAdapter;

typedef struct Model6CallbackAdapter {
  uint16_t (*x_callback)(void);
  void (*set_x_callback)(uint16_t);
  uint16_t (*offset_callback)(void);
  void (*set_offset_callback)(uint16_t);
  uint16_t (*value1_callback)(void);
  void (*set_value1_callback)(uint16_t);
  uint16_t (*val2_callback)(void);
  void (*set_val2_callback)(uint16_t);
  uint16_t (*val3_callback)(void);
  void (*set_val3_callback)(uint16_t);
  uint16_t (*val4_callback)(void);
  void (*set_val4_callback)(uint16_t);
  uint16_t (*val5_callback)(void);
  void (*set_val5_callback)(uint16_t);
  uint16_t (*val6_callback)(void);
  void (*set_val6_callback)(uint16_t);
  uint16_t (*val7_callback)(void);
  void (*set_val7_callback)(uint16_t);
  uint16_t (*val8_callback)(void);
  void (*set_val8_callback)(uint16_t);
  uint16_t (*val9_callback)(void);
  void (*set_val9_callback)(uint16_t);
  uint16_t (*val10_callback)(void);
  void (*set_val10_callback)(uint16_t);
  uint16_t (*val11_callback)(void);
  void (*set_val11_callback)(uint16_t);
  uint16_t (*val12_callback)(void);
  void (*set_val12_callback)(uint16_t);
  uint16_t (*val13_callback)(void);
  void (*set_val13_callback)(uint16_t);
  uint16_t (*val14_callback)(void);
  void (*set_val14_callback)(uint16_t);
  uint16_t (*val15_callback)(void);
  void (*set_val15_callback)(uint16_t);
  uint16_t (*val16_callback)(void);
  void (*set_val16_callback)(uint16_t);
  uint16_t (*val17_callback)(void);
  void (*set_val17_callback)(uint16_t);
  uint16_t (*val18_callback)(void);
  void (*set_val18_callback)(uint16_t);
  uint16_t (*val19_callback)(void);
  void (*set_val19_callback)(uint16_t);
  uint16_t (*val20_callback)(void);
  void (*set_val20_callback)(uint16_t);
  uint16_t (*val21_callback)(void);
  void (*set_val21_callback)(uint16_t);
  uint16_t (*val22_callback)(void);
  void (*set_val22_callback)(uint16_t);
  uint16_t (*val23_callback)(void);
  void (*set_val23_callback)(uint16_t);
  uint16_t (*val24_callback)(void);
  void (*set_val24_callback)(uint16_t);
  uint16_t (*val25_callback)(void);
  void (*set_val25_callback)(uint16_t);
  uint16_t (*val26_callback)(void);
  void (*set_val26_callback)(uint16_t);
  uint16_t (*val27_callback)(void);
  void (*set_val27_callback)(uint16_t);
  uint16_t (*val28_callback)(void);
  void (*set_val28_callback)(uint16_t);
  uint16_t (*val29_callback)(void);
  void (*set_val29_callback)(uint16_t);
  uint16_t (*val30_callback)(void);
  void (*set_val30_callback)(uint16_t);
  uint16_t (*val31_callback)(void);
  void (*set_val31_callback)(uint16_t);
  uint16_t (*val32_callback)(void);
  void (*set_val32_callback)(uint16_t);
  uint16_t (*val33_callback)(void);
  void (*set_val33_callback)(uint16_t);
  uint16_t (*val34_callback)(void);
  void (*set_val34_callback)(uint16_t);
  uint16_t (*val35_callback)(void);
  void (*set_val35_callback)(uint16_t);
  uint16_t (*val36_callback)(void);
  void (*set_val36_callback)(uint16_t);
  uint16_t (*val37_callback)(void);
  void (*set_val37_callback)(uint16_t);
  uint16_t (*val38_callback)(void);
  void (*set_val38_callback)(uint16_t);
  uint16_t (*val39_callback)(void);
  void (*set_val39_callback)(uint16_t);
  uint16_t (*val40_callback)(void);
  void (*set_val40_callback)(uint16_t);
  uint16_t (*val41_callback)(void);
  void (*set_val41_callback)(uint16_t);
  uint16_t (*val42_callback)(void);
  void (*set_val42_callback)(uint16_t);
  uint16_t (*val43_callback)(void);
  void (*set_val43_callback)(uint16_t);
  uint16_t (*val44_callback)(void);
  void (*set_val44_callback)(uint16_t);
  uint16_t (*val45_callback)(void);
  void (*set_val45_callback)(uint16_t);
  uint16_t (*val46_callback)(void);
  void (*set_val46_callback)(uint16_t);
  uint16_t (*val47_callback)(void);
  void (*set_val47_callback)(uint16_t);
  uint16_t (*val48_callback)(void);
  void (*set_val48_callback)(uint16_t);
  uint16_t (*val49_callback)(void);
  void (*set_val49_callback)(uint16_t);
  uint16_t (*val50_callback)(void);
  void (*set_val50_callback)(uint16_t);
  uint16_t (*val51_callback)(void);
  void (*set_val51_callback)(uint16_t);
  uint16_t (*val52_callback)(void);
  void (*set_val52_callback)(uint16_t);
  uint16_t (*val53_callback)(void);
  void (*set_val53_callback)(uint16_t);
  uint16_t (*val54_callback)(void);
  void (*set_val54_callback)(uint16_t);
  uint16_t (*val55_callback)(void);
  void (*set_val55_callback)(uint16_t);
  uint16_t (*val56_callback)(void);
  void (*set_val56_callback)(uint16_t);
  uint16_t (*val57_callback)(void);
  void (*set_val57_callback)(uint16_t);
  uint16_t (*val58_callback)(void);
  void (*set_val58_callback)(uint16_t);
  uint16_t (*val59_callback)(void);
  void (*set_val59_callback)(uint16_t);
  uint16_t (*val60_callback)(void);
  void (*set_val60_callback)(uint16_t);
  uint16_t (*val61_callback)(void);
  void (*set_val61_callback)(uint16_t);
  uint16_t (*val62_callback)(void);
  void (*set_val62_callback)(uint16_t);
  uint16_t (*val63_callback)(void);
  void (*set_val63_callback)(uint16_t);
  uint16_t (*val64_callback)(void);
  void (*set_val64_callback)(uint16_t);
  uint16_t (*val65_callback)(void);
  void (*set_val65_callback)(uint16_t);
  uint16_t (*val66_callback)(void);
  void (*set_val66_callback)(uint16_t);
  uint16_t (*val67_callback)(void);
  void (*set_val67_callback)(uint16_t);
  uint16_t (*val68_callback)(void);
  void (*set_val68_callback)(uint16_t);
  uint16_t (*val69_callback)(void);
  void (*set_val69_callback)(uint16_t);
  uint16_t (*val70_callback)(void);
  void (*set_val70_callback)(uint16_t);
  uint16_t (*val71_callback)(void);
  void (*set_val71_callback)(uint16_t);
  uint16_t (*val72_callback)(void);
  void (*set_val72_callback)(uint16_t);
  uint16_t (*val73_callback)(void);
  void (*set_val73_callback)(uint16_t);
  uint16_t (*val74_callback)(void);
  void (*set_val74_callback)(uint16_t);
  uint16_t (*val75_callback)(void);
  void (*set_val75_callback)(uint16_t);
  uint16_t (*val76_callback)(void);
  void (*set_val76_callback)(uint16_t);
  uint16_t (*val77_callback)(void);
  void (*set_val77_callback)(uint16_t);
  uint16_t (*val78_callback)(void);
  void (*set_val78_callback)(uint16_t);
  uint16_t (*val79_callback)(void);
  void (*set_val79_callback)(uint16_t);
  uint16_t (*val80_callback)(void);
  void (*set_val80_callback)(uint16_t);
  uint32_t (*timestamp_callback)(void);
  void (*set_timestamp_callback)(uint32_t);
  uint16_t (*milliseconds_callback)(void);
  void (*set_milliseconds_callback)(uint16_t);
  uint16_t (*sequence_callback)(void);
  void (*set_sequence_callback)(uint16_t);
  uint16_t (*role_callback)(void);
  void (*set_role_callback)(uint16_t);
  Alg (*algorithm_callback)(void);
  void (*set_algorithm_callback)(Alg);
  uint16_t (*n_callback)(void);
  void (*set_n_callback)(uint16_t);
} Model6CallbackAdapter;

typedef struct Model63001CallbackAdapter {
  uint16_t (*sunssf_1_callback)(void);
  uint16_t (*sunssf_2_callback)(void);
  uint16_t (*sunssf_3_callback)(void);
  uint16_t (*sunssf_4_callback)(void);
  int16_t (*int16_1_callback)(void);
  int16_t (*int16_2_callback)(void);
  int16_t (*int16_3_callback)(void);
  int16_t (*int16_4_callback)(void);
  void (*set_int16_4_callback)(int16_t);
  int16_t (*int16_5_callback)(void);
  int16_t (*int16_u_callback)(void);
  uint16_t (*uint16_1_callback)(void);
  uint16_t (*uint16_2_callback)(void);
  uint16_t (*uint16_3_callback)(void);
  uint16_t (*uint16_4_callback)(void);
  void (*set_uint16_4_callback)(uint16_t);
  uint16_t (*uint16_5_callback)(void);
  uint16_t (*uint16_u_callback)(void);
  uint16_t (*acc16_callback)(void);
  uint16_t (*acc16_u_callback)(void);
  uint16_t (*enum16_callback)(void);
  uint16_t (*enum16_u_callback)(void);
  uint16_t (*bitfield16_callback)(void);
  uint16_t (*bitfield16_u_callback)(void);
  int32_t (*int32_1_callback)(void);
  int32_t (*int32_2_callback)(void);
  int32_t (*int32_3_callback)(void);
  void (*set_int32_3_callback)(int32_t);
  int32_t (*int32_4_callback)(void);
  int32_t (*int32_5_callback)(void);
  int32_t (*int32_u_callback)(void);
  uint32_t (*uint32_1_callback)(void);
  uint32_t (*uint32_2_callback)(void);
  uint32_t (*uint32_3_callback)(void);
  void (*set_uint32_3_callback)(uint32_t);
  uint32_t (*uint32_4_callback)(void);
  uint32_t (*uint32_5_callback)(void);
  uint32_t (*uint32_u_callback)(void);
  uint32_t (*acc32_callback)(void);
  uint32_t (*acc32_u_callback)(void);
  uint32_t (*enum32_callback)(void);
  uint32_t (*enum32_u_callback)(void);
  uint32_t (*bitfield32_callback)(void);
  uint32_t (*bitfield32_u_callback)(void);
  Ipv4Addr (*ipaddr_callback)(void);
  void (*set_ipaddr_callback)(Ipv4Addr);
  Ipv4Addr (*ipaddr_u_callback)(void);
  int64_t (*int64_callback)(void);
  void (*set_int64_callback)(int64_t);
  int64_t (*int64_u_callback)(void);
  uint64_t (*acc64_callback)(void);
  uint64_t (*acc64_u_callback)(void);
  Ipv6Addr (*ipv6addr_callback)(void);
  Ipv6Addr (*ipv6addr_u_callback)(void);
  float (*float32_callback)(void);
  void (*set_float32_callback)(float);
  float (*float32_u_callback)(void);
  const char *(*string_callback)(void);
  void (*set_string_callback)(const char*);
  const char *(*string_u_callback)(void);
  uint16_t (*sunssf_5_callback)(void);
  uint16_t (*sunssf_6_callback)(void);
  uint16_t (*sunssf_7_callback)(void);
} Model63001CallbackAdapter;

typedef struct Model64001CallbackAdapter {
  uint16_t (*command_code_callback)(void);
  void (*set_command_code_callback)(uint16_t);
  uint16_t (*hardware_revision_callback)(void);
  uint16_t (*rs_fw_revision_callback)(void);
  uint16_t (*os_fw_revision_callback)(void);
  const char *(*product_revision_callback)(void);
  uint16_t (*boot_count_callback)(void);
  uint16_t (*dip_switches_callback)(void);
  uint16_t (*num_detected_sensors_callback)(void);
  uint16_t (*num_communicating_sensors_callback)(void);
  uint16_t (*system_status_callback)(void);
  uint16_t (*system_configuration_callback)(void);
  uint16_t (*led_blink_threshold_callback)(void);
  uint16_t (*led_on_threshold_callback)(void);
  uint16_t (*reserved_callback)(void);
  const char *(*location_string_callback)(void);
  uint16_t (*sensor_1_unit_id_callback)(void);
  uint16_t (*sensor_1_address_callback)(void);
  uint16_t (*sensor_1_os_version_callback)(void);
  const char *(*sensor_1_product_version_callback)(void);
  const char *(*sensor_1_serial_num_callback)(void);
  uint16_t (*sensor_2_unit_id_callback)(void);
  uint16_t (*sensor_2_address_callback)(void);
  uint16_t (*sensor_2_os_version_callback)(void);
  const char *(*sensor_2_product_version_callback)(void);
  const char *(*sensor_2_serial_num_callback)(void);
  uint16_t (*sensor_3_unit_id_callback)(void);
  uint16_t (*sensor_3_address_callback)(void);
  uint16_t (*sensor_3_os_version_callback)(void);
  const char *(*sensor_3_product_version_callback)(void);
  const char *(*sensor_3_serial_num_callback)(void);
  uint16_t (*sensor_4_unit_id_callback)(void);
  uint16_t (*sensor_4_address_callback)(void);
  uint16_t (*sensor_4_os_version_callback)(void);
  const char *(*sensor_4_product_version_callback)(void);
  const char *(*sensor_4_serial_num_callback)(void);
} Model64001CallbackAdapter;

typedef struct Model64020CallbackAdapter {
  int16_t (*aux_0_temperature_callback)(void);
  int16_t (*aux_1_temperature_callback)(void);
  int16_t (*aux_2_temperature_callback)(void);
  int16_t (*aux_3_temperature_callback)(void);
  int16_t (*aux_4_temperature_callback)(void);
  int16_t (*probe_temperature_callback)(void);
  int16_t (*main_temperature_callback)(void);
  uint16_t (*voltage_scale_factor_for_the_sensors_callback)(void);
  uint16_t (*current_scale_factor_for_the_sensors_callback)(void);
  uint16_t (*frequency_scale_factor_for_the_sensors_callback)(void);
  int16_t (*sensor1_voltage_callback)(void);
  int16_t (*sensor2_voltage_callback)(void);
  int16_t (*sensor3_voltage_callback)(void);
  int16_t (*sensor4_voltage_callback)(void);
  int16_t (*sensor5_voltage_callback)(void);
  int16_t (*sensor6_voltage_callback)(void);
  int16_t (*sensor7_voltage_callback)(void);
  int16_t (*sensor1_current_callback)(void);
  int16_t (*sensor2_current_callback)(void);
  int16_t (*sensor3_current_callback)(void);
  int16_t (*sensor4_current_callback)(void);
  int16_t (*sensor5_current_callback)(void);
  int16_t (*sensor6_current_callback)(void);
  int16_t (*sensor7_current_callback)(void);
  uint16_t (*sensor8_frequency_callback)(void);
  uint16_t (*relay_1_state_callback)(void);
  uint16_t (*relay_2_state_callback)(void);
  uint16_t (*relay_3_state_callback)(void);
  uint16_t (*reset_the_accumulators_callback)(void);
  uint16_t (*reset_the_system_callback)(void);
} Model64020CallbackAdapter;

typedef struct Model64101CallbackAdapter {
  uint16_t (*eltek_country_code_callback)(void);
  uint16_t (*eltek_feeding_phase_callback)(void);
  uint16_t (*eltek_apd_method_callback)(void);
  uint16_t (*eltek_apd_power_ref_callback)(void);
  uint16_t (*eltek_rps_method_callback)(void);
  uint16_t (*eltek_rps_q_ref_callback)(void);
  int16_t (*eltek_rps_cos_phi_ref_callback)(void);
} Model64101CallbackAdapter;

typedef struct Model64111CallbackAdapter {
  uint16_t (*port_number_callback)(void);
  uint16_t (*v_sf_callback)(void);
  uint16_t (*a_sf_callback)(void);
  uint16_t (*p_sf_callback)(void);
  uint16_t (*ah_sf_callback)(void);
  uint16_t (*kwh_sf_callback)(void);
  uint16_t (*battery_voltage_callback)(void);
  uint16_t (*array_voltage_callback)(void);
  uint16_t (*output_current_callback)(void);
  uint16_t (*array_current_callback)(void);
  ChargerSt (*operating_state_callback)(void);
  uint16_t (*output_wattage_callback)(void);
  uint16_t (*today_s_minimum_battery_voltage_callback)(void);
  uint16_t (*today_s_maximum_battery_voltage_callback)(void);
  uint16_t (*voc_callback)(void);
  uint16_t (*today_s_maximum_voc_callback)(void);
  uint16_t (*today_s_k_wh_callback)(void);
  uint16_t (*today_s_ah_callback)(void);
  uint16_t (*lifetime_k_wh_callback)(void);
  uint16_t (*lifetime_k_ah_callback)(void);
  uint16_t (*lifetime_maximum_output_wattage_callback)(void);
  uint16_t (*lifetime_maximum_battery_voltage_callback)(void);
  uint16_t (*lifetime_maximum_voc_voltage_callback)(void);
} Model64111CallbackAdapter;

typedef struct Model64112CallbackAdapter {
  uint16_t (*port_number_callback)(void);
  uint16_t (*v_sf_callback)(void);
  uint16_t (*c_sf_callback)(void);
  uint16_t (*h_sf_callback)(void);
  uint16_t (*p_sf_callback)(void);
  uint16_t (*ah_sf_callback)(void);
  uint16_t (*kwh_sf_callback)(void);
  uint16_t (*faults_callback)(void);
  uint16_t (*absorb_callback)(void);
  uint16_t (*absorb_time_callback)(void);
  uint16_t (*absorb_end_callback)(void);
  uint16_t (*rebulk_callback)(void);
  uint16_t (*float_callback)(void);
  uint16_t (*maximum_charge_callback)(void);
  uint16_t (*equalize_callback)(void);
  uint16_t (*equalize_time_callback)(void);
  uint16_t (*auto_equalize_interval_callback)(void);
  CcConfigMpptMode (*mppt_mode_callback)(void);
  CcConfigSweepWidth (*sweep_width_callback)(void);
  CcConfigSweepMax (*sweep_maximum_callback)(void);
  uint16_t (*u_pick_pwm_duty_cycle_callback)(void);
  CcConfigGridTie (*grid_tie_mode_callback)(void);
  CcConfigTempComp (*temp_comp_mode_callback)(void);
  uint16_t (*temp_comp_lower_limit_callback)(void);
  uint16_t (*temp_comp_upper_limit_callback)(void);
  CcConfigAutoRestart (*auto_restart_mode_callback)(void);
  uint16_t (*wakeup_voc_change_callback)(void);
  uint16_t (*snooze_mode_callback)(void);
  uint16_t (*wakeup_interval_callback)(void);
  CcConfigAuxMode (*aux_output_mode_callback)(void);
  CcConfigAuxControl (*aux_output_control_callback)(void);
  CcConfigAuxState (*aux_output_state_callback)(void);
  CcConfigAuxPolarity (*aux_output_polarity_callback)(void);
  uint16_t (*aux_low_battery_disconnect_callback)(void);
  uint16_t (*aux_low_battery_reconnect_callback)(void);
  uint16_t (*aux_low_battery_disconnect_delay_callback)(void);
  uint16_t (*aux_vent_fan_callback)(void);
  uint16_t (*aux_pv_trigger_callback)(void);
  uint16_t (*aux_pv_trigger_hold_time_callback)(void);
  uint16_t (*aux_night_light_threshold_callback)(void);
  uint16_t (*aux_night_light_on_time_callback)(void);
  uint16_t (*aux_night_light_on_hysteresis_callback)(void);
  uint16_t (*aux_night_light_off_hysteresis_callback)(void);
  uint16_t (*aux_error_output_low_battery_callback)(void);
  uint16_t (*aux_divert_hold_time_callback)(void);
  uint16_t (*aux_divert_delay_time_callback)(void);
  uint16_t (*aux_divert_relative_callback)(void);
  uint16_t (*aux_divert_hysteresis_callback)(void);
  uint16_t (*fm_cc_major_firmware_number_callback)(void);
  uint16_t (*fm_cc_mid_firmware_number_callback)(void);
  uint16_t (*fm_cc_minor_firmware_number_callback)(void);
  uint16_t (*set_data_log_day_offset_callback)(void);
  uint16_t (*current_data_log_day_offset_callback)(void);
  uint16_t (*data_log_daily_ah_callback)(void);
  uint16_t (*data_log_daily_k_wh_callback)(void);
  uint16_t (*data_log_daily_maximum_output_a_callback)(void);
  uint16_t (*data_log_daily_maximum_output_w_callback)(void);
  uint16_t (*data_log_daily_absorb_time_callback)(void);
  uint16_t (*data_log_daily_float_time_callback)(void);
  uint16_t (*data_log_daily_minimum_battery_callback)(void);
  uint16_t (*data_log_daily_maximum_battery_callback)(void);
  uint16_t (*data_log_daily_maximum_input_callback)(void);
  uint16_t (*data_log_clear_callback)(void);
  uint16_t (*data_log_clear_complement_callback)(void);
} Model64112CallbackAdapter;

typedef struct Model64410CallbackAdapter {
  uint16_t (*maximum_voltage_callback)(void);
  void (*set_maximum_voltage_callback)(uint16_t);
  uint16_t (*maximum_power_callback)(void);
  void (*set_maximum_power_callback)(uint16_t);
  uint16_t (*maximum_current_callback)(void);
  void (*set_maximum_current_callback)(uint16_t);
  Mode (*cv_or_cc_mode_callback)(void);
  void (*set_cv_or_cc_mode_callback)(Mode);
  Ena (*power_on_off_callback)(void);
  void (*set_power_on_off_callback)(Ena);
  Reset (*reset_device_callback)(void);
  void (*set_reset_device_callback)(Reset);
  uint16_t (*voltage_setpoint_callback)(void);
  void (*set_voltage_setpoint_callback)(uint16_t);
  uint16_t (*power_setpoint_callback)(void);
  void (*set_power_setpoint_callback)(uint16_t);
  uint16_t (*current_setpoint_callback)(void);
  void (*set_current_setpoint_callback)(uint16_t);
  En50530 (*en50530_mode_callback)(void);
  void (*set_en50530_mode_callback)(En50530);
  uint16_t (*en50530_mpp_voltage_callback)(void);
  void (*set_en50530_mpp_voltage_callback)(uint16_t);
  uint16_t (*en50530_mpp_power_callback)(void);
  void (*set_en50530_mpp_power_callback)(uint16_t);
  uint16_t (*irradiance_setpoint_callback)(void);
  void (*set_irradiance_setpoint_callback)(uint16_t);
  uint16_t (*voltage_slew_rate_callback)(void);
  void (*set_voltage_slew_rate_callback)(uint16_t);
  uint16_t (*power_slew_rate_callback)(void);
  void (*set_power_slew_rate_callback)(uint16_t);
  uint16_t (*current_slew_rate_callback)(void);
  void (*set_current_slew_rate_callback)(uint16_t);
  EnaProf (*enable_profile_callback)(void);
  void (*set_enable_profile_callback)(EnaProf);
  uint16_t (*profile_adoption_request_callback)(void);
  void (*set_profile_adoption_request_callback)(uint16_t);
  AdptProfRslt (*adopt_profile_result_callback)(void);
  int32_t (*measured_voltage_callback)(void);
  int32_t (*measured_power_callback)(void);
  int32_t (*measured_current_callback)(void);
  const char *(*errors_callback)(void);
  uint16_t (*number_of_points_callback)(void);
  uint16_t (*stored_profile_count_callback)(void);
  uint16_t (*power_scale_factor_callback)(void);
  void (*set_power_scale_factor_callback)(uint16_t);
  uint16_t (*voltage_scale_factor_callback)(void);
  void (*set_voltage_scale_factor_callback)(uint16_t);
  uint16_t (*current_scale_factor_callback)(void);
  void (*set_current_scale_factor_callback)(uint16_t);
  uint16_t (*irradiance_scale_factor_callback)(void);
  void (*set_irradiance_scale_factor_callback)(uint16_t);
  uint16_t (*time_scale_factor_callback)(void);
  void (*set_time_scale_factor_callback)(uint16_t);
  uint16_t (*voltage_slew_rate_scale_factor_callback)(void);
  void (*set_voltage_slew_rate_scale_factor_callback)(uint16_t);
  uint16_t (*power_slew_rate_scale_factor_callback)(void);
  void (*set_power_slew_rate_scale_factor_callback)(uint16_t);
  uint16_t (*current_slew_rate_scale_factor_callback)(void);
  void (*set_current_slew_rate_scale_factor_callback)(uint16_t);
  uint16_t (*percent_scale_factor_callback)(void);
  void (*set_percent_scale_factor_callback)(uint16_t);
} Model64410CallbackAdapter;

typedef struct Model64411CallbackAdapter {
  uint16_t (*active_phases_callback)(void);
  void (*set_active_phases_callback)(uint16_t);
  uint16_t (*phase_angle_callback)(void);
  void (*set_phase_angle_callback)(uint16_t);
  uint16_t (*nominal_voltage_callback)(void);
  void (*set_nominal_voltage_callback)(uint16_t);
  uint16_t (*maximum_voltage_callback)(void);
  void (*set_maximum_voltage_callback)(uint16_t);
  uint16_t (*maximum_current_callback)(void);
  void (*set_maximum_current_callback)(uint16_t);
  uint16_t (*frequency_callback)(void);
  void (*set_frequency_callback)(uint16_t);
  Output (*output_state_callback)(void);
  void (*set_output_state_callback)(Output);
  Relay (*relay_state_callback)(void);
  void (*set_relay_state_callback)(Relay);
  Regen (*regeneration_state_callback)(void);
  void (*set_regeneration_state_callback)(Regen);
  uint16_t (*voltage_setpoint_callback)(void);
  void (*set_voltage_setpoint_callback)(uint16_t);
  uint16_t (*voltage_setpoint_phase_a_callback)(void);
  void (*set_voltage_setpoint_phase_a_callback)(uint16_t);
  uint16_t (*voltage_setpoint_phase_b_callback)(void);
  void (*set_voltage_setpoint_phase_b_callback)(uint16_t);
  uint16_t (*voltage_setpoint_phase_c_callback)(void);
  void (*set_voltage_setpoint_phase_c_callback)(uint16_t);
  uint16_t (*frequency_slew_rate_callback)(void);
  void (*set_frequency_slew_rate_callback)(uint16_t);
  uint16_t (*voltage_slew_rate_callback)(void);
  void (*set_voltage_slew_rate_callback)(uint16_t);
  int32_t (*measured_voltage_phase_a_callback)(void);
  int32_t (*measured_voltage_phase_b_callback)(void);
  int32_t (*measured_voltage_phase_c_callback)(void);
  int32_t (*measured_frequency_callback)(void);
  int32_t (*measured_current_phase_a_callback)(void);
  int32_t (*measured_current_phase_b_callback)(void);
  int32_t (*measured_current_phase_c_callback)(void);
  const char *(*voltage_harmonics_phase_a_callback)(void);
  const char *(*voltage_harmonics_phase_b_callback)(void);
  const char *(*voltage_harmonics_phase_c_callback)(void);
  const char *(*current_harmonics_phase_a_callback)(void);
  const char *(*current_harmonics_phase_b_callback)(void);
  const char *(*current_harmonics_phase_c_callback)(void);
  const char *(*current_interharmonics_phase_a_callback)(void);
  const char *(*current_interharmonics_phase_b_callback)(void);
  const char *(*current_interharmonics_phase_c_callback)(void);
  uint16_t (*voltage_thd_phase_a_callback)(void);
  uint16_t (*voltage_thd_phase_b_callback)(void);
  uint16_t (*voltage_thd_phase_c_callback)(void);
  uint16_t (*current_thd_phase_a_callback)(void);
  uint16_t (*current_thd_phase_b_callback)(void);
  uint16_t (*current_thd_phase_c_callback)(void);
  EnaProf (*enable_profile_callback)(void);
  void (*set_enable_profile_callback)(EnaProf);
  ProfRslt (*profile_result_callback)(void);
  uint16_t (*stored_profile_count_callback)(void);
  uint16_t (*max_profile_point_count_callback)(void);
  uint16_t (*voltage_scale_factor_callback)(void);
  uint16_t (*current_scale_factor_callback)(void);
  uint16_t (*time_scale_factor_callback)(void);
  uint16_t (*frequency_scale_factor_callback)(void);
  uint16_t (*frequency_slew_rate_scale_factor_callback)(void);
  uint16_t (*voltage_slew_rate_scale_factor_callback)(void);
  uint16_t (*thd_scale_factor_callback)(void);
} Model64411CallbackAdapter;

typedef struct Model64412CallbackAdapter {
  DaManipulation (*da_manipulation_callback)(void);
  void (*set_da_manipulation_callback)(DaManipulation);
  FalsifyDeviceIdentity (*falsify_device_identity_callback)(void);
  void (*set_falsify_device_identity_callback)(FalsifyDeviceIdentity);
  MeasPAlwaysNameplate (*meas_p_always_nameplate_callback)(void);
  void (*set_meas_p_always_nameplate_callback)(MeasPAlwaysNameplate);
  MeasQAlwaysMinimum (*meas_q_always_minimum_callback)(void);
  void (*set_meas_q_always_minimum_callback)(MeasQAlwaysMinimum);
  MeasQAlwaysMaximum (*meas_q_always_maximum_callback)(void);
  void (*set_meas_q_always_maximum_callback)(MeasQAlwaysMaximum);
  MeasQAlwaysZero (*meas_q_always_zero_callback)(void);
  void (*set_meas_q_always_zero_callback)(MeasQAlwaysZero);
  MeasZeroP (*meas_zero_p_callback)(void);
  void (*set_meas_zero_p_callback)(MeasZeroP);
  MeasInvertQ (*meas_invert_q_callback)(void);
  void (*set_meas_invert_q_callback)(MeasInvertQ);
  MeasLowV (*meas_low_v_callback)(void);
  void (*set_meas_low_v_callback)(MeasLowV);
  MeasHighV (*meas_high_v_callback)(void);
  void (*set_meas_high_v_callback)(MeasHighV);
  MeasLowL1v (*meas_low_l1_v_callback)(void);
  void (*set_meas_low_l1_v_callback)(MeasLowL1v);
  MeasHighL1v (*meas_high_l1_v_callback)(void);
  void (*set_meas_high_l1_v_callback)(MeasHighL1v);
  MeasLowF (*meas_low_f_callback)(void);
  void (*set_meas_low_f_callback)(MeasLowF);
  MeasHighF (*meas_high_f_callback)(void);
  void (*set_meas_high_f_callback)(MeasHighF);
  MeasLowAmps (*meas_low_amps_callback)(void);
  void (*set_meas_low_amps_callback)(MeasLowAmps);
  MeasHighAmps (*meas_high_amps_callback)(void);
  void (*set_meas_high_amps_callback)(MeasHighAmps);
  MeasHighS (*meas_high_s_callback)(void);
  void (*set_meas_high_s_callback)(MeasHighS);
  MeasLowS (*meas_low_s_callback)(void);
  void (*set_meas_low_s_callback)(MeasLowS);
  MeasHighQ (*meas_high_q_callback)(void);
  void (*set_meas_high_q_callback)(MeasHighQ);
  MeasLowQ (*meas_low_q_callback)(void);
  void (*set_meas_low_q_callback)(MeasLowQ);
  MeasLowPf (*meas_low_pf_callback)(void);
  void (*set_meas_low_pf_callback)(MeasLowPf);
  MeasLowReversedPf (*meas_low_reversed_pf_callback)(void);
  void (*set_meas_low_reversed_pf_callback)(MeasLowReversedPf);
  NameplateHighP (*nameplate_high_p_callback)(void);
  void (*set_nameplate_high_p_callback)(NameplateHighP);
  NameplateLowP (*nameplate_low_p_callback)(void);
  void (*set_nameplate_low_p_callback)(NameplateLowP);
  NameplateHighS (*nameplate_high_s_callback)(void);
  void (*set_nameplate_high_s_callback)(NameplateHighS);
  NameplateLowS (*nameplate_low_s_callback)(void);
  void (*set_nameplate_low_s_callback)(NameplateLowS);
  NameplateHighQ (*nameplate_high_q_callback)(void);
  void (*set_nameplate_high_q_callback)(NameplateHighQ);
  NameplateLowQ (*nameplate_low_q_callback)(void);
  void (*set_nameplate_low_q_callback)(NameplateLowQ);
  NameplateHighNomV (*nameplate_high_nom_v_callback)(void);
  void (*set_nameplate_high_nom_v_callback)(NameplateHighNomV);
  NameplateLowNomV (*nameplate_low_nom_v_callback)(void);
  void (*set_nameplate_low_nom_v_callback)(NameplateLowNomV);
  NameplateLowAmps (*nameplate_low_amps_callback)(void);
  void (*set_nameplate_low_amps_callback)(NameplateLowAmps);
  NameplateLowVarmaxinj (*nameplate_low_varmaxinj_callback)(void);
  void (*set_nameplate_low_varmaxinj_callback)(NameplateLowVarmaxinj);
  NameplateLowVarmaxabs (*nameplate_low_varmaxabs_callback)(void);
  void (*set_nameplate_low_varmaxabs_callback)(NameplateLowVarmaxabs);
  NameplateLowPf (*nameplate_low_pf_callback)(void);
  void (*set_nameplate_low_pf_callback)(NameplateLowPf);
  SettingsHighNomV (*settings_high_nom_v_callback)(void);
  void (*set_settings_high_nom_v_callback)(SettingsHighNomV);
  SettingsLowAmps (*settings_low_amps_callback)(void);
  void (*set_settings_low_amps_callback)(SettingsLowAmps);
  SettingsHighP (*settings_high_p_callback)(void);
  void (*set_settings_high_p_callback)(SettingsHighP);
  SettingsLowP (*settings_low_p_callback)(void);
  void (*set_settings_low_p_callback)(SettingsLowP);
  SettingsHighVaMax (*settings_high_va_max_callback)(void);
  void (*set_settings_high_va_max_callback)(SettingsHighVaMax);
  SettingsHighVarmaxinj (*settings_high_varmaxinj_callback)(void);
  void (*set_settings_high_varmaxinj_callback)(SettingsHighVarmaxinj);
  SettingsHighVarmaxabs (*settings_high_varmaxabs_callback)(void);
  void (*set_settings_high_varmaxabs_callback)(SettingsHighVarmaxabs);
  ChangeCommonModelId (*change_common_model_id_callback)(void);
  void (*set_change_common_model_id_callback)(ChangeCommonModelId);
  ChangeCommonModelLength (*change_common_model_length_callback)(void);
  void (*set_change_common_model_length_callback)(ChangeCommonModelLength);
} Model64412CallbackAdapter;

typedef struct Model64413CallbackAdapter {
  uint16_t (*iv_length_callback)(void);
  uint16_t (*poa_irradiance_callback)(void);
  uint16_t (*irr_sf_callback)(void);
} Model64413CallbackAdapter;

typedef struct Model64414CallbackAdapter {
  const char *(*time_offset_callback)(void);
  float (*temperature_callback)(void);
  void (*set_temperature_callback)(float);
  const char *(*grid_model_source_callback)(void);
  void (*set_grid_model_source_callback)(const char*);
  const char *(*irradiance_model_source_callback)(void);
  void (*set_irradiance_model_source_callback)(const char*);
  float (*irradiance_callback)(void);
  void (*set_irradiance_callback)(float);
  float (*grid_voltage_a_callback)(void);
  void (*set_grid_voltage_a_callback)(float);
  float (*grid_voltage_b_callback)(void);
  void (*set_grid_voltage_b_callback)(float);
  float (*grid_voltage_c_callback)(void);
  void (*set_grid_voltage_c_callback)(float);
  float (*grid_frequency_callback)(void);
  void (*set_grid_frequency_callback)(float);
} Model64414CallbackAdapter;

typedef struct Model64415CallbackAdapter {
  LogEventEna (*log_event_mode_enable_callback)(void);
  void (*set_log_event_mode_enable_callback)(LogEventEna);
  HttpMsg (*http_message_mode_enable_callback)(void);
  void (*set_http_message_mode_enable_callback)(HttpMsg);
  Comm004Cert (*comm_004_certificate_callback)(void);
  void (*set_comm_004_certificate_callback)(Comm004Cert);
  const char *(*subscribed_resource_url_callback)(void);
  void (*set_subscribed_resource_url_callback)(const char*);
  SubscriptionEna (*subscribtion_enable_callback)(void);
  void (*set_subscribtion_enable_callback)(SubscriptionEna);
} Model64415CallbackAdapter;

typedef struct Model7CallbackAdapter {
  uint16_t (*request_sequence_callback)(void);
  Sts (*status_callback)(void);
  uint32_t (*timestamp_callback)(void);
  uint16_t (*milliseconds_callback)(void);
  uint16_t (*sequence_callback)(void);
  Alm (*alarm_callback)(void);
  Alg (*algorithm_callback)(void);
  uint16_t (*n_callback)(void);
  void (*set_n_callback)(uint16_t);
} Model7CallbackAdapter;

typedef struct Model701CallbackAdapter {
  AcType (*ac_wiring_type_callback)(void);
  St (*operating_state_callback)(void);
  InvSt (*inverter_state_callback)(void);
  ConnSt (*grid_connection_state_callback)(void);
  uint32_t (*alarm_bitfield_callback)(void);
  uint32_t (*der_operational_characteristics_callback)(void);
  int16_t (*active_power_callback)(void);
  int16_t (*apparent_power_callback)(void);
  int16_t (*reactive_power_callback)(void);
  int16_t (*power_factor_callback)(void);
  int16_t (*total_ac_current_callback)(void);
  uint16_t (*voltage_ll_callback)(void);
  uint16_t (*voltage_ln_callback)(void);
  uint32_t (*frequency_callback)(void);
  uint64_t (*total_energy_injected_callback)(void);
  uint64_t (*total_energy_absorbed_callback)(void);
  uint64_t (*total_reactive_energy_inj_callback)(void);
  uint64_t (*total_reactive_energy_abs_callback)(void);
  int16_t (*ambient_temperature_callback)(void);
  int16_t (*cabinet_temperature_callback)(void);
  int16_t (*heat_sink_temperature_callback)(void);
  int16_t (*transformer_temperature_callback)(void);
  int16_t (*igbt_mosfet_temperature_callback)(void);
  int16_t (*other_temperature_callback)(void);
  int16_t (*watts_l1_callback)(void);
  int16_t (*va_l1_callback)(void);
  int16_t (*var_l1_callback)(void);
  int16_t (*pf_l1_callback)(void);
  int16_t (*amps_l1_callback)(void);
  uint16_t (*phase_voltage_l1_l2_callback)(void);
  uint16_t (*phase_voltage_l1_n_callback)(void);
  uint64_t (*total_watt_hours_inj_l1_callback)(void);
  uint64_t (*total_watt_hours_abs_l1_callback)(void);
  uint64_t (*total_var_hours_inj_l1_callback)(void);
  uint64_t (*total_var_hours_abs_l1_callback)(void);
  int16_t (*watts_l2_callback)(void);
  int16_t (*va_l2_callback)(void);
  int16_t (*var_l2_callback)(void);
  int16_t (*pf_l2_callback)(void);
  int16_t (*amps_l2_callback)(void);
  uint16_t (*phase_voltage_l2_l3_callback)(void);
  uint16_t (*phase_voltage_l2_n_callback)(void);
  uint64_t (*total_watt_hours_inj_l2_callback)(void);
  uint64_t (*total_watt_hours_abs_l2_callback)(void);
  uint64_t (*total_var_hours_inj_l2_callback)(void);
  uint64_t (*total_var_hours_abs_l2_callback)(void);
  int16_t (*watts_l3_callback)(void);
  int16_t (*va_l3_callback)(void);
  int16_t (*var_l3_callback)(void);
  int16_t (*pf_l3_callback)(void);
  int16_t (*amps_l3_callback)(void);
  uint16_t (*phase_voltage_l3_l1_callback)(void);
  uint16_t (*phase_voltage_l3_n_callback)(void);
  uint64_t (*total_watt_hours_inj_l3_callback)(void);
  uint64_t (*total_watt_hours_abs_l3_callback)(void);
  uint64_t (*total_var_hours_inj_l3_callback)(void);
  uint64_t (*total_var_hours_abs_l3_callback)(void);
  uint16_t (*throttling_in_pct_callback)(void);
  uint32_t (*throttle_source_information_callback)(void);
  uint16_t (*current_scale_factor_callback)(void);
  uint16_t (*voltage_scale_factor_callback)(void);
  uint16_t (*frequency_scale_factor_callback)(void);
  uint16_t (*active_power_scale_factor_callback)(void);
  uint16_t (*power_factor_scale_factor_callback)(void);
  uint16_t (*apparent_power_scale_factor_callback)(void);
  uint16_t (*reactive_power_scale_factor_callback)(void);
  uint16_t (*active_energy_scale_factor_callback)(void);
  uint16_t (*reactive_energy_scale_factor_callback)(void);
  uint16_t (*temperature_scale_factor_callback)(void);
  const char *(*manufacturer_alarm_info_callback)(void);
} Model701CallbackAdapter;

typedef struct Model703CallbackAdapter {
  Es (*permit_enter_service_callback)(void);
  void (*set_permit_enter_service_callback)(Es);
  uint16_t (*enter_service_voltage_high_callback)(void);
  void (*set_enter_service_voltage_high_callback)(uint16_t);
  uint16_t (*enter_service_voltage_low_callback)(void);
  void (*set_enter_service_voltage_low_callback)(uint16_t);
  uint32_t (*enter_service_frequency_high_callback)(void);
  void (*set_enter_service_frequency_high_callback)(uint32_t);
  uint32_t (*enter_service_frequency_low_callback)(void);
  void (*set_enter_service_frequency_low_callback)(uint32_t);
  uint32_t (*enter_service_delay_time_callback)(void);
  void (*set_enter_service_delay_time_callback)(uint32_t);
  uint32_t (*enter_service_random_delay_callback)(void);
  void (*set_enter_service_random_delay_callback)(uint32_t);
  uint32_t (*enter_service_ramp_time_callback)(void);
  void (*set_enter_service_ramp_time_callback)(uint32_t);
  uint32_t (*enter_service_delay_remaining_callback)(void);
  uint16_t (*voltage_scale_factor_callback)(void);
  uint16_t (*frequency_scale_factor_callback)(void);
} Model703CallbackAdapter;

typedef struct Model704CallbackAdapter {
  PfwInjEna (*power_factor_enable_w_inj_enable_callback)(void);
  void (*set_power_factor_enable_w_inj_enable_callback)(PfwInjEna);
  PfwInjEnaRvrt (*power_factor_reversion_enable_w_inj_callback)(void);
  void (*set_power_factor_reversion_enable_w_inj_callback)(PfwInjEnaRvrt);
  uint32_t (*pf_reversion_time_w_inj_callback)(void);
  void (*set_pf_reversion_time_w_inj_callback)(uint32_t);
  uint32_t (*pf_reversion_time_rem_w_inj_callback)(void);
  PfwAbsEna (*power_factor_enable_w_abs_enable_callback)(void);
  void (*set_power_factor_enable_w_abs_enable_callback)(PfwAbsEna);
  PfwAbsEnaRvrt (*power_factor_reversion_enable_w_abs_callback)(void);
  void (*set_power_factor_reversion_enable_w_abs_callback)(PfwAbsEnaRvrt);
  uint32_t (*pf_reversion_time_w_abs_callback)(void);
  void (*set_pf_reversion_time_w_abs_callback)(uint32_t);
  uint32_t (*pf_reversion_time_rem_w_abs_callback)(void);
  WMaxLimPctEna (*limit_max_power_pct_enable_callback)(void);
  void (*set_limit_max_power_pct_enable_callback)(WMaxLimPctEna);
  uint16_t (*limit_max_power_pct_setpoint_callback)(void);
  void (*set_limit_max_power_pct_setpoint_callback)(uint16_t);
  uint16_t (*reversion_limit_max_power_pct_callback)(void);
  void (*set_reversion_limit_max_power_pct_callback)(uint16_t);
  WMaxLimPctEnaRvrt (*reversion_limit_max_power_pct_enable_callback)(void);
  void (*set_reversion_limit_max_power_pct_enable_callback)(WMaxLimPctEnaRvrt);
  uint32_t (*limit_max_power_pct_reversion_time_callback)(void);
  void (*set_limit_max_power_pct_reversion_time_callback)(uint32_t);
  uint32_t (*limit_max_power_pct_rev_time_rem_callback)(void);
  WSetEna (*set_active_power_enable_callback)(void);
  void (*set_set_active_power_enable_callback)(WSetEna);
  WSetMod (*set_active_power_mode_callback)(void);
  void (*set_set_active_power_mode_callback)(WSetMod);
  int32_t (*active_power_setpoint_w_callback)(void);
  void (*set_active_power_setpoint_w_callback)(int32_t);
  int32_t (*reversion_active_power_w_callback)(void);
  void (*set_reversion_active_power_w_callback)(int32_t);
  int16_t (*active_power_setpoint_pct_callback)(void);
  void (*set_active_power_setpoint_pct_callback)(int16_t);
  int16_t (*reversion_active_power_pct_callback)(void);
  void (*set_reversion_active_power_pct_callback)(int16_t);
  WSetEnaRvrt (*reversion_active_power_enable_callback)(void);
  void (*set_reversion_active_power_enable_callback)(WSetEnaRvrt);
  uint32_t (*active_power_reversion_time_callback)(void);
  void (*set_active_power_reversion_time_callback)(uint32_t);
  uint32_t (*active_power_rev_time_rem_callback)(void);
  VarSetEna (*set_reactive_power_enable_callback)(void);
  void (*set_set_reactive_power_enable_callback)(VarSetEna);
  VarSetMod (*set_reactive_power_mode_callback)(void);
  void (*set_set_reactive_power_mode_callback)(VarSetMod);
  VarSetPri (*reactive_power_priority_callback)(void);
  void (*set_reactive_power_priority_callback)(VarSetPri);
  int32_t (*reactive_power_setpoint_vars_callback)(void);
  void (*set_reactive_power_setpoint_vars_callback)(int32_t);
  int32_t (*reversion_reactive_power_vars_callback)(void);
  void (*set_reversion_reactive_power_vars_callback)(int32_t);
  int16_t (*reactive_power_setpoint_pct_callback)(void);
  void (*set_reactive_power_setpoint_pct_callback)(int16_t);
  int16_t (*reversion_reactive_power_pct_callback)(void);
  void (*set_reversion_reactive_power_pct_callback)(int16_t);
  VarSetEnaRvrt (*reversion_reactive_power_enable_callback)(void);
  void (*set_reversion_reactive_power_enable_callback)(VarSetEnaRvrt);
  uint32_t (*reactive_power_reversion_time_callback)(void);
  void (*set_reactive_power_reversion_time_callback)(uint32_t);
  uint32_t (*reactive_power_rev_time_rem_callback)(void);
  uint16_t (*normal_ramp_rate_callback)(void);
  void (*set_normal_ramp_rate_callback)(uint16_t);
  WRmpRef (*normal_ramp_rate_reference_callback)(void);
  void (*set_normal_ramp_rate_reference_callback)(WRmpRef);
  uint16_t (*reactive_power_ramp_rate_callback)(void);
  void (*set_reactive_power_ramp_rate_callback)(uint16_t);
  AntiIslEna (*anti_islanding_enable_callback)(void);
  void (*set_anti_islanding_enable_callback)(AntiIslEna);
  uint16_t (*power_factor_scale_factor_callback)(void);
  uint16_t (*limit_max_power_scale_factor_callback)(void);
  uint16_t (*active_power_scale_factor_callback)(void);
  uint16_t (*active_power_pct_scale_factor_callback)(void);
  uint16_t (*reactive_power_scale_factor_callback)(void);
  uint16_t (*reactive_power_pct_scale_factor_callback)(void);
} Model704CallbackAdapter;

typedef struct Model705CallbackAdapter {
  Ena (*der_volt_var_module_enable_callback)(void);
  void (*set_der_volt_var_module_enable_callback)(Ena);
  uint16_t (*adopt_curve_request_callback)(void);
  void (*set_adopt_curve_request_callback)(uint16_t);
  AdptCrvRslt (*adopt_curve_result_callback)(void);
  uint16_t (*number_of_points_callback)(void);
  uint16_t (*stored_curve_count_callback)(void);
  uint32_t (*reversion_timeout_callback)(void);
  void (*set_reversion_timeout_callback)(uint32_t);
  uint32_t (*reversion_time_remaining_callback)(void);
  uint16_t (*reversion_curve_callback)(void);
  void (*set_reversion_curve_callback)(uint16_t);
  uint16_t (*voltage_scale_factor_callback)(void);
  uint16_t (*var_scale_factor_callback)(void);
  uint16_t (*open_loop_scale_factor_callback)(void);
} Model705CallbackAdapter;

typedef struct Model706CallbackAdapter {
  Ena (*der_volt_watt_module_enable_callback)(void);
  void (*set_der_volt_watt_module_enable_callback)(Ena);
  uint16_t (*adopt_curve_request_callback)(void);
  void (*set_adopt_curve_request_callback)(uint16_t);
  AdptCrvRslt (*adopt_curve_result_callback)(void);
  uint16_t (*number_of_points_callback)(void);
  uint16_t (*stored_curve_count_callback)(void);
  uint32_t (*reversion_timeout_callback)(void);
  void (*set_reversion_timeout_callback)(uint32_t);
  uint32_t (*reversion_time_remaining_callback)(void);
  uint16_t (*reversion_curve_callback)(void);
  void (*set_reversion_curve_callback)(uint16_t);
  uint16_t (*voltage_scale_factor_callback)(void);
  uint16_t (*watt_scale_factor_callback)(void);
  uint16_t (*open_loop_scale_factor_callback)(void);
} Model706CallbackAdapter;

typedef struct Model707CallbackAdapter {
  Ena (*der_trip_lv_module_enable_callback)(void);
  void (*set_der_trip_lv_module_enable_callback)(Ena);
  uint16_t (*adopt_curve_request_callback)(void);
  void (*set_adopt_curve_request_callback)(uint16_t);
  AdptCrvRslt (*adopt_curve_result_callback)(void);
  uint16_t (*number_of_points_callback)(void);
  uint16_t (*stored_curve_count_callback)(void);
  uint16_t (*voltage_scale_factor_callback)(void);
  uint16_t (*time_point_scale_factor_callback)(void);
} Model707CallbackAdapter;

typedef struct Model708CallbackAdapter {
  Ena (*der_trip_hv_module_enable_callback)(void);
  void (*set_der_trip_hv_module_enable_callback)(Ena);
  uint16_t (*adopt_curve_request_callback)(void);
  void (*set_adopt_curve_request_callback)(uint16_t);
  AdptCrvRslt (*adopt_curve_result_callback)(void);
  uint16_t (*number_of_points_callback)(void);
  uint16_t (*stored_curve_count_callback)(void);
  uint16_t (*voltage_scale_factor_callback)(void);
  uint16_t (*time_point_scale_factor_callback)(void);
} Model708CallbackAdapter;

typedef struct Model709CallbackAdapter {
  Ena (*der_trip_lf_module_enable_callback)(void);
  void (*set_der_trip_lf_module_enable_callback)(Ena);
  uint16_t (*adopt_curve_request_callback)(void);
  void (*set_adopt_curve_request_callback)(uint16_t);
  AdptCrvRslt (*adopt_curve_result_callback)(void);
  uint16_t (*number_of_points_callback)(void);
  uint16_t (*stored_curve_count_callback)(void);
  uint16_t (*frequency_scale_factor_callback)(void);
  uint16_t (*time_point_scale_factor_callback)(void);
} Model709CallbackAdapter;

typedef struct Model710CallbackAdapter {
  Ena (*der_trip_hf_module_enable_callback)(void);
  void (*set_der_trip_hf_module_enable_callback)(Ena);
  uint16_t (*adopt_curve_request_callback)(void);
  void (*set_adopt_curve_request_callback)(uint16_t);
  AdptCrvRslt (*adopt_curve_result_callback)(void);
  uint16_t (*number_of_points_callback)(void);
  uint16_t (*stored_curve_count_callback)(void);
  uint16_t (*frequency_scale_factor_callback)(void);
  uint16_t (*time_point_scale_factor_callback)(void);
} Model710CallbackAdapter;

typedef struct Model711CallbackAdapter {
  Ena (*der_frequency_droop_module_enable_callback)(void);
  void (*set_der_frequency_droop_module_enable_callback)(Ena);
  uint16_t (*set_active_control_request_callback)(void);
  void (*set_set_active_control_request_callback)(uint16_t);
  AdptCtlRslt (*set_active_control_result_callback)(void);
  uint16_t (*stored_control_count_callback)(void);
  uint32_t (*reversion_timeout_callback)(void);
  void (*set_reversion_timeout_callback)(uint32_t);
  uint32_t (*reversion_time_left_callback)(void);
  uint16_t (*reversion_control_callback)(void);
  void (*set_reversion_control_callback)(uint16_t);
  uint16_t (*deadband_scale_factor_callback)(void);
  uint16_t (*frequency_change_scale_factor_callback)(void);
  uint16_t (*open_loop_scale_factor_callback)(void);
} Model711CallbackAdapter;

typedef struct Model712CallbackAdapter {
  Ena (*der_watt_var_module_enable_callback)(void);
  void (*set_der_watt_var_module_enable_callback)(Ena);
  uint16_t (*set_active_curve_request_callback)(void);
  void (*set_set_active_curve_request_callback)(uint16_t);
  AdptCrvRslt (*set_active_curve_result_callback)(void);
  uint16_t (*number_of_points_callback)(void);
  uint16_t (*stored_curve_count_callback)(void);
  uint32_t (*reversion_timeout_callback)(void);
  void (*set_reversion_timeout_callback)(uint32_t);
  uint32_t (*reversion_time_left_callback)(void);
  uint16_t (*reversion_curve_callback)(void);
  void (*set_reversion_curve_callback)(uint16_t);
  uint16_t (*active_power_scale_factor_callback)(void);
  uint16_t (*var_scale_factor_callback)(void);
} Model712CallbackAdapter;

typedef struct Model713CallbackAdapter {
  uint16_t (*energy_rating_callback)(void);
  uint16_t (*energy_available_callback)(void);
  uint16_t (*state_of_charge_callback)(void);
  uint16_t (*state_of_health_callback)(void);
  Sta (*status_callback)(void);
  uint16_t (*energy_scale_factor_callback)(void);
  uint16_t (*percent_scale_factor_callback)(void);
} Model713CallbackAdapter;

typedef struct Model714CallbackAdapter {
  uint32_t (*port_alarms_callback)(void);
  uint16_t (*number_of_ports_callback)(void);
  int16_t (*dc_current_callback)(void);
  int16_t (*dc_power_callback)(void);
  uint64_t (*dc_energy_injected_callback)(void);
  uint64_t (*dc_energy_absorbed_callback)(void);
  uint16_t (*dc_current_scale_factor_callback)(void);
  uint16_t (*dc_voltage_scale_factor_callback)(void);
  uint16_t (*dc_power_scale_factor_callback)(void);
  uint16_t (*dc_energy_scale_factor_callback)(void);
  uint16_t (*temperature_scale_factor_callback)(void);
} Model714CallbackAdapter;

typedef struct Model715CallbackAdapter {
  LocRemCtl (*control_mode_callback)(void);
  uint32_t (*der_heartbeat_callback)(void);
  uint32_t (*controller_heartbeat_callback)(void);
  void (*set_controller_heartbeat_callback)(uint32_t);
  uint16_t (*alarm_reset_callback)(void);
  void (*set_alarm_reset_callback)(uint16_t);
  OpCtl (*set_operation_callback)(void);
  void (*set_set_operation_callback)(OpCtl);
} Model715CallbackAdapter;

typedef struct Model8CallbackAdapter {
  Fmt (*format_callback)(void);
  uint16_t (*n_callback)(void);
} Model8CallbackAdapter;

typedef struct Model801CallbackAdapter {
  uint16_t (*deprecated_model_callback)(void);
} Model801CallbackAdapter;

typedef struct Model802CallbackAdapter {
  uint16_t (*nameplate_charge_capacity_callback)(void);
  uint16_t (*nameplate_energy_capacity_callback)(void);
  uint16_t (*nameplate_max_charge_rate_callback)(void);
  uint16_t (*nameplate_max_discharge_rate_callback)(void);
  uint16_t (*self_discharge_rate_callback)(void);
  uint16_t (*nameplate_max_so_c_callback)(void);
  uint16_t (*nameplate_min_so_c_callback)(void);
  uint16_t (*max_reserve_percent_callback)(void);
  void (*set_max_reserve_percent_callback)(uint16_t);
  uint16_t (*min_reserve_percent_callback)(void);
  void (*set_min_reserve_percent_callback)(uint16_t);
  uint16_t (*state_of_charge_callback)(void);
  uint16_t (*depth_of_discharge_callback)(void);
  uint16_t (*state_of_health_callback)(void);
  uint32_t (*cycle_count_callback)(void);
  ChaSt (*charge_status_callback)(void);
  LocRemCtl (*control_mode_callback)(void);
  uint16_t (*battery_heartbeat_callback)(void);
  uint16_t (*controller_heartbeat_callback)(void);
  void (*set_controller_heartbeat_callback)(uint16_t);
  uint16_t (*alarm_reset_callback)(void);
  void (*set_alarm_reset_callback)(uint16_t);
  Typ (*battery_type_callback)(void);
  State (*state_of_the_battery_bank_callback)(void);
  uint16_t (*vendor_battery_bank_state_callback)(void);
  uint32_t (*warranty_date_callback)(void);
  uint32_t (*battery_event_1_bitfield_callback)(void);
  uint32_t (*battery_event_2_bitfield_callback)(void);
  uint32_t (*vendor_event_bitfield_1_callback)(void);
  uint32_t (*vendor_event_bitfield_2_callback)(void);
  uint16_t (*external_battery_voltage_callback)(void);
  uint16_t (*max_battery_voltage_callback)(void);
  uint16_t (*min_battery_voltage_callback)(void);
  uint16_t (*max_cell_voltage_callback)(void);
  uint16_t (*max_cell_voltage_string_callback)(void);
  uint16_t (*max_cell_voltage_module_callback)(void);
  uint16_t (*min_cell_voltage_callback)(void);
  uint16_t (*min_cell_voltage_string_callback)(void);
  uint16_t (*min_cell_voltage_module_callback)(void);
  uint16_t (*average_cell_voltage_callback)(void);
  int16_t (*total_dc_current_callback)(void);
  uint16_t (*max_charge_current_callback)(void);
  uint16_t (*max_discharge_current_callback)(void);
  int16_t (*total_power_callback)(void);
  ReqInvState (*inverter_state_request_callback)(void);
  int16_t (*battery_power_request_callback)(void);
  SetOp (*set_operation_callback)(void);
  void (*set_set_operation_callback)(SetOp);
  SetInvState (*set_inverter_state_callback)(void);
  void (*set_set_inverter_state_callback)(SetInvState);
  uint16_t (*ah_rtg_sf_callback)(void);
  uint16_t (*wh_rtg_sf_callback)(void);
  uint16_t (*w_cha_dis_cha_max_sf_callback)(void);
  uint16_t (*dis_cha_rte_sf_callback)(void);
  uint16_t (*so_c_sf_callback)(void);
  uint16_t (*do_d_sf_callback)(void);
  uint16_t (*so_h_sf_callback)(void);
  uint16_t (*v_sf_callback)(void);
  uint16_t (*cell_v_sf_callback)(void);
  uint16_t (*a_sf_callback)(void);
  uint16_t (*a_max_sf_callback)(void);
  uint16_t (*w_sf_callback)(void);
} Model802CallbackAdapter;

typedef struct Model803CallbackAdapter {
  uint16_t (*string_count_callback)(void);
  uint16_t (*connected_string_count_callback)(void);
  int16_t (*max_module_temperature_callback)(void);
  uint16_t (*max_module_temperature_string_callback)(void);
  uint16_t (*max_module_temperature_module_callback)(void);
  int16_t (*min_module_temperature_callback)(void);
  uint16_t (*min_module_temperature_string_callback)(void);
  uint16_t (*min_module_temperature_module_callback)(void);
  int16_t (*average_module_temperature_callback)(void);
  uint16_t (*max_string_voltage_callback)(void);
  uint16_t (*max_string_voltage_string_callback)(void);
  uint16_t (*min_string_voltage_callback)(void);
  uint16_t (*min_string_voltage_string_callback)(void);
  uint16_t (*average_string_voltage_callback)(void);
  int16_t (*max_string_current_callback)(void);
  uint16_t (*max_string_current_string_callback)(void);
  int16_t (*min_string_current_callback)(void);
  uint16_t (*min_string_current_string_callback)(void);
  int16_t (*average_string_current_callback)(void);
  uint16_t (*battery_cell_balancing_count_callback)(void);
  uint16_t (*cell_v_sf_callback)(void);
  uint16_t (*mod_tmp_sf_callback)(void);
  uint16_t (*a_sf_callback)(void);
  uint16_t (*so_h_sf_callback)(void);
  uint16_t (*so_c_sf_callback)(void);
  uint16_t (*v_sf_callback)(void);
} Model803CallbackAdapter;

typedef struct Model804CallbackAdapter {
  uint16_t (*string_index_callback)(void);
  uint16_t (*module_count_callback)(void);
  uint32_t (*string_status_callback)(void);
  ConFail (*connection_failure_reason_callback)(void);
  uint16_t (*string_cell_balancing_count_callback)(void);
  uint16_t (*string_state_of_charge_callback)(void);
  uint16_t (*string_depth_of_discharge_callback)(void);
  uint32_t (*string_cycle_count_callback)(void);
  uint16_t (*string_state_of_health_callback)(void);
  int16_t (*string_current_callback)(void);
  uint16_t (*string_voltage_callback)(void);
  uint16_t (*max_cell_voltage_callback)(void);
  uint16_t (*max_cell_voltage_module_callback)(void);
  uint16_t (*min_cell_voltage_callback)(void);
  uint16_t (*min_cell_voltage_module_callback)(void);
  uint16_t (*average_cell_voltage_callback)(void);
  int16_t (*max_module_temperature_callback)(void);
  uint16_t (*max_module_temperature_module_callback)(void);
  int16_t (*min_module_temperature_callback)(void);
  uint16_t (*min_module_temperature_module_callback)(void);
  int16_t (*average_module_temperature_callback)(void);
  uint32_t (*contactor_status_callback)(void);
  uint32_t (*string_event_1_callback)(void);
  uint32_t (*string_event_2_callback)(void);
  uint32_t (*vendor_event_bitfield_1_callback)(void);
  uint32_t (*vendor_event_bitfield_2_callback)(void);
  uint16_t (*enable_disable_string_callback)(void);
  void (*set_enable_disable_string_callback)(uint16_t);
  SetCon (*connect_disconnect_string_callback)(void);
  void (*set_connect_disconnect_string_callback)(SetCon);
  uint16_t (*so_c_sf_callback)(void);
  uint16_t (*so_h_sf_callback)(void);
  uint16_t (*do_d_sf_callback)(void);
  uint16_t (*a_sf_callback)(void);
  uint16_t (*v_sf_callback)(void);
  uint16_t (*cell_v_sf_callback)(void);
  uint16_t (*mod_tmp_sf_callback)(void);
} Model804CallbackAdapter;

typedef struct Model805CallbackAdapter {
  uint16_t (*string_index_callback)(void);
  uint16_t (*module_index_callback)(void);
  uint16_t (*module_cell_count_callback)(void);
  uint16_t (*module_so_c_callback)(void);
  uint16_t (*depth_of_discharge_callback)(void);
  uint16_t (*module_so_h_callback)(void);
  uint32_t (*cycle_count_callback)(void);
  uint16_t (*module_voltage_callback)(void);
  uint16_t (*max_cell_voltage_callback)(void);
  uint16_t (*max_cell_voltage_cell_callback)(void);
  uint16_t (*min_cell_voltage_callback)(void);
  uint16_t (*min_cell_voltage_cell_callback)(void);
  uint16_t (*average_cell_voltage_callback)(void);
  int16_t (*max_cell_temperature_callback)(void);
  uint16_t (*max_cell_temperature_cell_callback)(void);
  int16_t (*min_cell_temperature_callback)(void);
  uint16_t (*min_cell_temperature_cell_callback)(void);
  int16_t (*average_cell_temperature_callback)(void);
  uint16_t (*balanced_cell_count_callback)(void);
  const char *(*serial_number_callback)(void);
  uint16_t (*so_c_sf_callback)(void);
  uint16_t (*so_h_sf_callback)(void);
  uint16_t (*do_d_sf_callback)(void);
  uint16_t (*v_sf_callback)(void);
  uint16_t (*cell_v_sf_callback)(void);
  uint16_t (*tmp_sf_callback)(void);
} Model805CallbackAdapter;

typedef struct Model806CallbackAdapter {
  uint16_t (*battery_points_to_be_determined_callback)(void);
} Model806CallbackAdapter;

typedef struct Model807CallbackAdapter {
  uint16_t (*string_index_callback)(void);
  uint16_t (*module_count_callback)(void);
  uint16_t (*connected_module_count_callback)(void);
  uint16_t (*max_module_voltage_callback)(void);
  uint16_t (*max_module_voltage_module_callback)(void);
  uint16_t (*min_module_voltage_callback)(void);
  uint16_t (*min_module_voltage_module_callback)(void);
  uint16_t (*average_module_voltage_callback)(void);
  uint16_t (*max_cell_voltage_callback)(void);
  uint16_t (*max_cell_voltage_module_callback)(void);
  uint16_t (*max_cell_voltage_stack_callback)(void);
  uint16_t (*min_cell_voltage_callback)(void);
  uint16_t (*min_cell_voltage_module_callback)(void);
  uint16_t (*min_cell_voltage_stack_callback)(void);
  uint16_t (*average_cell_voltage_callback)(void);
  int16_t (*max_temperature_callback)(void);
  uint16_t (*max_temperature_module_callback)(void);
  int16_t (*min_temperature_callback)(void);
  uint16_t (*min_temperature_module_callback)(void);
  int16_t (*average_temperature_callback)(void);
  uint32_t (*string_event_1_callback)(void);
  uint32_t (*string_event_2_callback)(void);
  uint32_t (*vendor_event_bitfield_1_callback)(void);
  uint32_t (*vendor_event_bitfield_2_callback)(void);
  uint16_t (*mod_v_sf_callback)(void);
  uint16_t (*cell_v_sf_callback)(void);
  uint16_t (*tmp_sf_callback)(void);
  uint16_t (*so_c_sf_callback)(void);
  uint16_t (*ocv_sf_callback)(void);
} Model807CallbackAdapter;

typedef struct Model808CallbackAdapter {
  uint16_t (*module_points_to_be_determined_callback)(void);
} Model808CallbackAdapter;

typedef struct Model809CallbackAdapter {
  uint16_t (*stack_points_to_be_determined_callback)(void);
} Model809CallbackAdapter;

typedef struct SunspecCallbackAdapters {
  const struct Model1CallbackAdapter *model_1_adapter;
  const struct Model10CallbackAdapter *model_10_adapter;
  const struct Model101CallbackAdapter *model_101_adapter;
  const struct Model102CallbackAdapter *model_102_adapter;
  const struct Model103CallbackAdapter *model_103_adapter;
  const struct Model11CallbackAdapter *model_11_adapter;
  const struct Model111CallbackAdapter *model_111_adapter;
  const struct Model112CallbackAdapter *model_112_adapter;
  const struct Model113CallbackAdapter *model_113_adapter;
  const struct Model12CallbackAdapter *model_12_adapter;
  const struct Model120CallbackAdapter *model_120_adapter;
  const struct Model121CallbackAdapter *model_121_adapter;
  const struct Model122CallbackAdapter *model_122_adapter;
  const struct Model123CallbackAdapter *model_123_adapter;
  const struct Model124CallbackAdapter *model_124_adapter;
  const struct Model125CallbackAdapter *model_125_adapter;
  const struct Model126CallbackAdapter *model_126_adapter;
  const struct Model127CallbackAdapter *model_127_adapter;
  const struct Model128CallbackAdapter *model_128_adapter;
  const struct Model129CallbackAdapter *model_129_adapter;
  const struct Model13CallbackAdapter *model_13_adapter;
  const struct Model130CallbackAdapter *model_130_adapter;
  const struct Model131CallbackAdapter *model_131_adapter;
  const struct Model132CallbackAdapter *model_132_adapter;
  const struct Model133CallbackAdapter *model_133_adapter;
  const struct Model134CallbackAdapter *model_134_adapter;
  const struct Model135CallbackAdapter *model_135_adapter;
  const struct Model136CallbackAdapter *model_136_adapter;
  const struct Model137CallbackAdapter *model_137_adapter;
  const struct Model138CallbackAdapter *model_138_adapter;
  const struct Model139CallbackAdapter *model_139_adapter;
  const struct Model140CallbackAdapter *model_140_adapter;
  const struct Model141CallbackAdapter *model_141_adapter;
  const struct Model142CallbackAdapter *model_142_adapter;
  const struct Model143CallbackAdapter *model_143_adapter;
  const struct Model144CallbackAdapter *model_144_adapter;
  const struct Model145CallbackAdapter *model_145_adapter;
  const struct Model15CallbackAdapter *model_15_adapter;
  const struct Model16CallbackAdapter *model_16_adapter;
  const struct Model160CallbackAdapter *model_160_adapter;
  const struct Model17CallbackAdapter *model_17_adapter;
  const struct Model18CallbackAdapter *model_18_adapter;
  const struct Model19CallbackAdapter *model_19_adapter;
  const struct Model2CallbackAdapter *model_2_adapter;
  const struct Model201CallbackAdapter *model_201_adapter;
  const struct Model202CallbackAdapter *model_202_adapter;
  const struct Model203CallbackAdapter *model_203_adapter;
  const struct Model204CallbackAdapter *model_204_adapter;
  const struct Model211CallbackAdapter *model_211_adapter;
  const struct Model212CallbackAdapter *model_212_adapter;
  const struct Model213CallbackAdapter *model_213_adapter;
  const struct Model214CallbackAdapter *model_214_adapter;
  const struct Model220CallbackAdapter *model_220_adapter;
  const struct Model3CallbackAdapter *model_3_adapter;
  const struct Model305CallbackAdapter *model_305_adapter;
  const struct Model306CallbackAdapter *model_306_adapter;
  const struct Model307CallbackAdapter *model_307_adapter;
  const struct Model308CallbackAdapter *model_308_adapter;
  const struct Model4CallbackAdapter *model_4_adapter;
  const struct Model401CallbackAdapter *model_401_adapter;
  const struct Model402CallbackAdapter *model_402_adapter;
  const struct Model403CallbackAdapter *model_403_adapter;
  const struct Model404CallbackAdapter *model_404_adapter;
  const struct Model5CallbackAdapter *model_5_adapter;
  const struct Model501CallbackAdapter *model_501_adapter;
  const struct Model502CallbackAdapter *model_502_adapter;
  const struct Model6CallbackAdapter *model_6_adapter;
  const struct Model63001CallbackAdapter *model_63001_adapter;
  const struct Model64001CallbackAdapter *model_64001_adapter;
  const struct Model64020CallbackAdapter *model_64020_adapter;
  const struct Model64101CallbackAdapter *model_64101_adapter;
  const struct Model64111CallbackAdapter *model_64111_adapter;
  const struct Model64112CallbackAdapter *model_64112_adapter;
  const struct Model64410CallbackAdapter *model_64410_adapter;
  const struct Model64411CallbackAdapter *model_64411_adapter;
  const struct Model64412CallbackAdapter *model_64412_adapter;
  const struct Model64413CallbackAdapter *model_64413_adapter;
  const struct Model64414CallbackAdapter *model_64414_adapter;
  const struct Model64415CallbackAdapter *model_64415_adapter;
  const struct Model7CallbackAdapter *model_7_adapter;
  const struct Model701CallbackAdapter *model_701_adapter;
  const struct Model703CallbackAdapter *model_703_adapter;
  const struct Model704CallbackAdapter *model_704_adapter;
  const struct Model705CallbackAdapter *model_705_adapter;
  const struct Model706CallbackAdapter *model_706_adapter;
  const struct Model707CallbackAdapter *model_707_adapter;
  const struct Model708CallbackAdapter *model_708_adapter;
  const struct Model709CallbackAdapter *model_709_adapter;
  const struct Model710CallbackAdapter *model_710_adapter;
  const struct Model711CallbackAdapter *model_711_adapter;
  const struct Model712CallbackAdapter *model_712_adapter;
  const struct Model713CallbackAdapter *model_713_adapter;
  const struct Model714CallbackAdapter *model_714_adapter;
  const struct Model715CallbackAdapter *model_715_adapter;
  const struct Model8CallbackAdapter *model_8_adapter;
  const struct Model801CallbackAdapter *model_801_adapter;
  const struct Model802CallbackAdapter *model_802_adapter;
  const struct Model803CallbackAdapter *model_803_adapter;
  const struct Model804CallbackAdapter *model_804_adapter;
  const struct Model805CallbackAdapter *model_805_adapter;
  const struct Model806CallbackAdapter *model_806_adapter;
  const struct Model807CallbackAdapter *model_807_adapter;
  const struct Model808CallbackAdapter *model_808_adapter;
  const struct Model809CallbackAdapter *model_809_adapter;
} SunspecCallbackAdapters;

typedef struct SunspecService {
  struct SunspecCallbackAdapters adapters;
} SunspecService;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

extern int32_t printf(const char *format, ...);

extern int32_t printf(const char *format, ...);

extern int32_t printf(const char *format, ...);

int32_t sunspec_service_handle_request(const struct SunspecService *service,
                                       uint16_t address,
                                       uint16_t length,
                                       uint16_t *response_buffer);

void sunspec_service_init(struct SunspecService *service,
                          const struct Model1CallbackAdapter *adapter);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  /* SUNSPEC_MODBUS_CODEC_H */
