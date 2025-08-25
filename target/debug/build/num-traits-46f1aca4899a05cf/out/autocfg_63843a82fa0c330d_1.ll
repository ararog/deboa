; ModuleID = 'autocfg_63843a82fa0c330d_1.81aa3e6c3700e087-cgu.0'
source_filename = "autocfg_63843a82fa0c330d_1.81aa3e6c3700e087-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@alloc_f93507f8ba4b5780b14b2c2584609be0 = private unnamed_addr constant [8 x i8] c"\00\00\00\00\00\00\F0?", align 8
@alloc_ef0a1f828f3393ef691f2705e817091c = private unnamed_addr constant [8 x i8] c"\00\00\00\00\00\00\00@", align 8

; core::f64::<impl f64>::total_cmp
; Function Attrs: inlinehint nonlazybind uwtable
define internal i8 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$9total_cmp17hb40846a88911acc4E"(ptr align 8 %self, ptr align 8 %other) unnamed_addr #0 {
start:
  %_6 = alloca [8 x i8], align 8
  %_3 = alloca [8 x i8], align 8
  %_5 = load double, ptr %self, align 8
  %_4 = bitcast double %_5 to i64
  store i64 %_4, ptr %_3, align 8
  %_8 = load double, ptr %other, align 8
  %_7 = bitcast double %_8 to i64
  store i64 %_7, ptr %_6, align 8
  %_13 = load i64, ptr %_3, align 8
  %_12 = ashr i64 %_13, 63
  %_10 = lshr i64 %_12, 1
  %0 = load i64, ptr %_3, align 8
  %1 = xor i64 %0, %_10
  store i64 %1, ptr %_3, align 8
  %_18 = load i64, ptr %_6, align 8
  %_17 = ashr i64 %_18, 63
  %_15 = lshr i64 %_17, 1
  %2 = load i64, ptr %_6, align 8
  %3 = xor i64 %2, %_15
  store i64 %3, ptr %_6, align 8
  %_19 = load i64, ptr %_3, align 8
  %_20 = load i64, ptr %_6, align 8
  %_0 = call i8 @llvm.scmp.i8.i64(i64 %_19, i64 %_20)
  ret i8 %_0
}

; autocfg_63843a82fa0c330d_1::probe
; Function Attrs: nonlazybind uwtable
define void @_ZN26autocfg_63843a82fa0c330d_15probe17haf46a4189d97ce42E() unnamed_addr #1 {
start:
; call core::f64::<impl f64>::total_cmp
  %_1 = call i8 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$9total_cmp17hb40846a88911acc4E"(ptr align 8 @alloc_f93507f8ba4b5780b14b2c2584609be0, ptr align 8 @alloc_ef0a1f828f3393ef691f2705e817091c)
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i8 @llvm.scmp.i8.i64(i64, i64) #2

attributes #0 = { inlinehint nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #1 = { nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #2 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }

!llvm.module.flags = !{!0, !1}
!llvm.ident = !{!2}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
!2 = !{!"rustc version 1.88.0 (6b00bc388 2025-06-23)"}
