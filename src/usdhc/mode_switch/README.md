# switch to HS200

To switch to HS200, the Driver must perform the following steps:

1. Select the chip (through sending CMD7) and make sure it is unlocked (through CMD42).
2. Read the DEVICE_TYPE [196] field of the Extended CSD register to validate if the chip supports HS200 at the IO voltage appropriate for both host and chip (through CMD8).
3. Read the DRIVER_STRENGTH [197] field of the Extended CSD register to find the supported chip Driver Strengths (through CMD8). The default setting is 50 Ohm. The Host Designer might simulate its specific system, using a device driver models. Host can select the optimal Driver Type that might drive the host system load at the required operating frequency with the minimal noise generated.

    **NOTE**
    This step can be skipped if changes of Driver strength is not needed.

4. Check the setting of PROT_CTRL[DTW], make sure the data transfer width is set as expected. Make sure the I/O pad voltage is 1.8V and corresponding drive strength is set to proper value. Set HS200 bit and Driver Strength value in the HS_TIMING [185] field of the Extended CSD register by issuing CMD6. If the host attempts to write an invalid value, the HS_TIMING byte is not changed, the HS200 interface timing is not enabled, the Driver Strength is not changed, and the SWITCH_ERROR field is set. After the chip responds with R1, it might assert Busy signal. Once the busy signal gets de-asserted, the host may send a SEND_STATUS Command (CMD13) using  the HS200 timing and after it receives a Trans State indication and No Error it means that the chip is set to HS200 timing and the Driver Strength is set to the selected settings.
5. At this point, the host can set the frequency to ≤ 200 MHz.
6. The host might invoke the HS200 standard tuning procedure, by sending CMD21 to the chip.

    **NOTE**
    The host must switch to the required bus width before starting the tuning operation to allow the tuning sequence to be done using the proper bus operating conditions.


## Standard Tuning Procedure

By default, lower frequency operation, a fixed sampling clock is used to receive signals
on CMD and DAT[3:0]. Before using the HS200, HS400, SDR104, or SDR50 modes, the
Host Driver executes the tuning procedure at the mode switch sequence.

1. Issue uSDHC SW reset, set SYS_CTRL[RSTT] to 1.
2. Set VEND_SPEC[FRC_SDCLK_ON] to 1.
3. Set TUNING_CTRL[DIS_CMD_CHK_FOR_STD_TUNING] to 1.
4. Start the tuning procedure by setting TUNING_CTRL[STD_TUNING_EN] and MIX_CTRL[EXE_TUNE] to 1.
5. Issue CMD19(SD)/ CMD21(eMMC) with the proper Command Transfer Type (CMD_XFR_TYP) and Mixer Control (MIX_CTRL) settings.
6. Wait for uSDHC BRR (Buffer Read Ready) interrupt signal is 1.
7. Check MIX_CTRL[EXE_TUNE]. If MIX_CTRL[EXE_TUNE] = 1, repeat 5~6. If MIX_CTRL[EXE_TUNE] = 0, standard tuning has completed, or the tuning has not completed within 40 attempts. The Host Driver might abort this loop if the number of loops exceeds 40 or 150ms timeout occurs. In this case, a fixed sampling clock should be used, (AUTOCMD12_ERR_STATUS[SMP_CLK_SEL] = 0).
8. Sampling Clock Select, AUTOCMD12_ERR_STATUS[SMP_CLK_SEL] , is valid after MIX_CTRL[EXE_TUNE] has changed from 1 to 0. AUTOCMD12_ERR_STATUS[SMP_CLK_SEL] = 1, indicates tuning procedure passed. AUTOCMD12_ERR_STATUS[SMP_CLK_SEL] = 0, indicates tuning procedure failed. The tuning result is applied to the delay chain, CLK Tuning Control and Status (CLK_TUNE_CTRL_STATUS) [30:16], upon successful tuning procedure completion.
9. Clear VEND_SPEC[FRC_SDCLK_ON].
10. Set MIX_CTRL[AUTO_TUNE_EN] to 1.

While the tuning sequence is being performed, the Host Controller does not generate
interrupts (including Command Complete), except Buffer Read Ready. CMD19 response
errors are not indicated.

Writing AUTOCMD12_ERR_STATUS[SMP_CLK_SEL] to 0 forces the Host Controller to use a fixed sampling clock and resets the tuning circuit of the Host Controller.

### NOTE

- There could be slight difference on delay cell delay in each project, and this difference can lead to different loop number needed. TUNING_CTRL[TUNING_STEP] can be used to control how may taps are to be added for each step.
- Manual tuning might be required in cases where standard tuning is not able to close passing window with CMD19/21 transfer fail.