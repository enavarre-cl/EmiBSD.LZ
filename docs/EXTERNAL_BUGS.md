# External bugs

Bugs found while porting and testing that belong to someone else: OpenBSD's C at the pin
(`reference/PINNED.md`), QEMU and its bundled firmware (EDK2), and the host tools (macOS,
LLVM). Started by the user on 2026-10-09: "un log de bugs externos, donde también documentes
errores que encuentres en OpenBSD que debamos comunicarles".

This file records them and how the port lives with each; it reports nothing. Reporting
upstream (OpenBSD's bugs@ list or tech@, QEMU's GitLab, EDK2's GitHub) is outward-facing and
done only by the user or with the user's explicit OK, entry by entry.

## Rules

- Every bug found in OpenBSD's C (a slip the port fixes, bounds or works around), in QEMU,
  EDK2 or a host tool gets an entry here in the same commit that meets it (the port's
  `## Deviations` and `ports.toml` note say what the port does; this file says what is wrong
  upstream and what to tell them). Agents list them in their hand-back.
- An entry is evidence, not opinion: the file and line at the pin (`path:line`, 12-hex pin),
  or the QEMU version and command line, what happens, the lines seen, and what was checked.
  `Verified` says how sure we are: `reproduced` (seen run, e.g. on OpenBSD 8.0 through
  `diff-openbsd probe`), `read` (the C was read and the slip is plain), or `claimed` (an
  agent's note, not checked yet).
- Before an OpenBSD entry is proposed for reporting, check it against OpenBSD -current (the
  pin may be behind) and against the man pages and the hardware's specification it relies on.
- `Status`: `open` (not reported), `to report` (checked, the user may send it), `reported
  <link>`, `fixed upstream <commit>`, `not a bug` (kept, with why).
- An entry the port works around in its test environment (a retry, a restated criterion)
  names that workaround, so it is removed when upstream fixes the bug.

## Summary

Reach says who can trigger an entry: `remote` (a network peer), `user` (an unprivileged local
user), `device` (a malicious or broken USB, disk or network device, or a crafted image a user
mounts or attaches), `root`, `firmware/hardware` (buggy or malicious firmware or hardware, or
just the hardware being present), `alloc-failure` (only when an allocation fails). The
OpenBSD rows (EXT-9 on) are ordered by reach, then severity (memory corruption, crash, leak,
wrong result, cosmetic); their details are under the subsystem headings below. "Same fault"
means EmiBSD's code has it too ("Bugs the port reproduces").

| Id | Where | What | Reach | Severity | Verified | Port's handling | Status |
|---|---|---|---|---|---|---|---|
| EXT-1 | EDK2 (QEMU 11.1.2's `edk2-aarch64-code.fd`) | `UhciDxe` ASSERT before any OS loads, ~1 in 5 arm64 boots with a UHCI controller | — | — | reproduced | `xtask smoke` boots once more | open |
| EXT-2 | macOS (host) | an `xtask` process freezes before `main` (`_dyld_start`) | — | — | reproduced, cause unknown | `smoke-all`'s watchdog | open, investigating |
| EXT-3 | QEMU TCG (arm64, MTTCG, 2+ vCPUs) | a `sev` can be lost, so a `wfe` never wakes | — | — | reproduced | generic timer event stream on (deviation, M11e) | open (the user's decision pending) |
| EXT-186 | `sys/dev/pci/eap.c:850` | `eap_set_params` refuses every unsigned encoding (case falls into `default`) | user | wrong result | read | same result | open |
| EXT-184 | `sys/dev/pckbc/pms.c:2088` | Elantech v1 parity table inverted on every re-enable | device | wrong result | read | faithful | open |
| EXT-185 | `sys/dev/pckbc/pms.c:1726` | Elantech v1..v3 read-back loops take a failed command for success | device | wrong result | read | faithful (v1's buffer zeroed) | open |
| EXT-182 | `sys/dev/isa/spkr.c:190` | 26 or more dots after a note divide by zero in `playtone` | root | crash | reproduced | same fault (Rust's division panics as the C traps) | open |
| EXT-181 | `sys/dev/pckbc/pckbd.c:499` | opening the keyboard fails when the opener runs on the CPU that takes IRQ1 | root | wrong result | reproduced | same fault; smokes retry the open | open |
| EXT-4 | OpenBSD `sys/dev/pci/ehci_pci.c:128` | 16-bit write to the 32-bit EHCI `USBINTR` register; OpenBSD 8.0 panics on arm64 QEMU | firmware/hardware | crash | reproduced | faithful; criterion restated (M16b) | open |
| EXT-5 | QEMU I/O APIC with OpenBSD's cold routing (amd64) | ehci's INTx raised while its pin is masked and edge-triggered is dropped, the controller never interrupts | — | — | reproduced | faithful; criterion restated (M16b) | open, to analyse |
| EXT-6 | QEMU `pci-ohci` or OpenBSD ohci(4) | a write to a stick through ohci halts the controller | — | — | reproduced | faithful; criterion restated (M16b) | open, to analyse |
| EXT-7 | QEMU `tulip` or OpenBSD dc(4) | `failed to force tx to idle state`, `watchdog timeout`, no traffic | — | — | reproduced | faithful; criterion restated (M16c) | open, to analyse |
| EXT-8 | QEMU `igb` or OpenBSD em(4) | the 82576 attaches and is active but receives nothing, both archs | — | — | reproduced | faithful; criterion restated (M16c) | open, to analyse |
| EXT-192 | QEMU `megasas`/`megasas-gen2` or OpenBSD mfi(4) | `mfi0: could not initialize firmware`, `mfi0: can't attach`, both archs | firmware/hardware | wrong result | reproduced | mfi(4) not ported: moved to M17 by the user | open, to analyse |
| EXT-193 | QEMU `am53c974`/`dc390` or OpenBSD pcscp(4) | the disk's INQUIRY comes back empty after a Check Condition, sd1 unusable | firmware/hardware | wrong result | reproduced | ncr53c9x/pcscp not ported: moved to M17 by the user | open, to analyse |
| EXT-194 | QEMU `ufs` or OpenBSD ufshci(4) | the LU attaches as sd1, then the boot hangs before `root on` (amd64) | firmware/hardware | wrong result (boot hangs) | reproduced | ufshci not ported: moved to M17 by the user | open, to analyse |
| EXT-195 | QEMU `pvscsi` or OpenBSD vmwpvs(4) | `vmwpvs0: get configuration failed`: no scsibus | firmware/hardware | wrong result | reproduced | faithful; criterion restated (M16a) | open, to analyse |
| EXT-196 | QEMU `isa-fdc` or OpenBSD fd(4) (`sys/dev/isa/fd.c:843`) | a read that ends at a cylinder's end reports st0 SEEK END; fd(4) calls it a hard error | firmware/hardware | wrong result | reproduced | faithful (`smoke-fd` expects the error) | open, to analyse |
| EXT-197 | QEMU `isa-fdc` and OpenBSD fd(4) (`sys/dev/isa/fd.c:233`) | drive 0 is never found: the reset's "ready changed" status lands in the recalibrate's | firmware/hardware | wrong result | reproduced | faithful; the image sits in drive B (`storage.rs`) | open, to analyse |
| EXT-23 | `sys/netinet/tcp_output.c:1195` | `tcp_softtso_chop` uses the IP header pointer after `m_pullup` | remote | memory corruption | claimed | fixed | open |
| EXT-27 | `sys/nfs/nfs_serv.c:1151` | `nfsrv_create` frees the name buffer twice | remote | memory corruption | read | fixed | open |
| EXT-137 | `sys/dev/ic/dc.c:2153` | receive trusts the descriptor's frame length | remote | memory corruption | read | fixed | open |
| EXT-29 | `sys/nfs/nfs_vnops.c:2942` | `nfs_lookitup` calls `vput(NULL)` | remote | crash | read | fixed | open |
| EXT-9 | `sys/netinet6/ip6_forward.c:318` | IPsec-only forwarding drop leaks the packet | remote | leak | read | fixed | open |
| EXT-10 | `sys/netinet6/icmp6.c:1206` | redirect on a detached interface leaks the packet | remote | leak | read | fixed | open |
| EXT-24 | `sys/nfs/nfs_socket.c:988` | client leaks a denied RPC's reply | remote | leak | read | fixed | open |
| EXT-25 | `sys/nfs/nfs_serv.c:751` | `nfsrv_read` leaks the first reply when the read fails | remote | leak | read | fixed | open |
| EXT-26 | `sys/nfs/nfs_serv.c:1207` | CREATE GUARDED on an existing name leaves it locked | remote | leak | read | fixed | open |
| EXT-28 | `sys/nfs/nfs_serv.c:1740` | `nfsrv_rename` error path leaks a vnode reference and a name | remote | leak | read | fixed | open |
| EXT-30 | `sys/nfs/nfs_syscalls.c:517` | `nfsrv_zapsock` leaks a partial TCP record | remote | leak | read | same fault (see below) | open |
| EXT-31 | `sys/nfs/krpc_subr.c:432` | `krpc_call` leaks the reply on EBADRPC | remote | leak | read | fixed | open |
| EXT-180 | `sys/lib/libsa/netudp.c:220` | checksum over `uh_ulen` bytes past the datagram | remote | cosmetic | read | fixed | open |
| EXT-66 | `sys/isofs/udf/udf_subr.c:204` | VAT copy keeps a file entry the cache frees | user | memory corruption | read | fixed | open |
| EXT-159 | `sys/dev/rasops/rasops8.c:370` | underlined space written two lines up, before the frame buffer | user | memory corruption | read | same fault (see below) | open |
| EXT-38 | `sys/miscfs/fuse/fuse_vnops.c:333` | `fusefs_close` from `unp_gc` dereferences a NULL proc | user | crash | read | fixed | open |
| EXT-57 | `sys/isofs/udf/udf_vnops.c:1271` | unaligned read hands out bytes past the buffer | user | crash | read | fixed | open |
| EXT-103 | `sys/dev/usb/uftdi.c:1223` | 2232H divides by a speed of 0 | user | crash | read | fixed | open |
| EXT-51 | `sys/ntfs/ntfs_subr.c:1415` | failed `uiomove` leaves a buffer busy | user | leak | read | fixed | open |
| EXT-15 | `sys/kern/kern_proc.c:275` | new session inherits stale `s_verauth*` (doas persist) | user | wrong result | read | fixed | open |
| EXT-43 | `sys/miscfs/fuse/fuse_lookup.c:161` | uninitialised vtype written on a RENAME to `..` | user | wrong result | read | fixed | open |
| EXT-58 | `sys/isofs/udf/udf_vnops.c:1252` | embedded file data ignores the offset | user | wrong result | read | fixed | open |
| EXT-61 | `sys/isofs/udf/udf_vnops.c:816` | `udf_readdir` returns ERESTART for ever | user | wrong result | read | same fault (see below) | open |
| EXT-73 | `sys/scsi/cd.c:1409` | `cd_play_tracks` indexes the TOC with a user track | user | wrong result | read | fixed | open |
| EXT-158 | `sys/dev/wscons/wsemul_vt100.c:1087` | eleventh CSI argument touches `args[10]` | user | cosmetic | read | fixed | open |
| EXT-160 | `sys/dev/rasops/rasops8.c:136` | glyph rows read 4 bytes past the font | user | cosmetic | read | fixed | open |
| EXT-46 | `sys/ntfs/ntfs_compr.c:98` | 4 KB blocks written into a smaller unit buffer | device | memory corruption | read | panics instead | open |
| EXT-49 | `sys/ntfs/ntfs_subr.c:876` | index root buffer sized by `ir_size`, filled by `va_datalen` | device | memory corruption | read | fixed | open |
| EXT-53 | `sys/ntfs/ntfs_vfsops.c:408` | `$AttrDef` name copied without a bound | device | memory corruption | read | fixed | open |
| EXT-71 | `sys/scsi/cd.c:1581` | READ TOC length from the drive overruns the TOC | device | memory corruption | read | fixed | open |
| EXT-72 | `sys/scsi/cd.c:913` | CDIOREADTOCENTRYS converts entries past the TOC | device | memory corruption | read | fixed | open |
| EXT-85 | `sys/dev/usb/ugen.c:863` | isochronous ring pointer wraps before the buffer | device | memory corruption | read | fixed | open |
| EXT-87 | `sys/dev/usb/umass.c:1274` | CSW residue over the length gives memcpy a negative size | device | memory corruption | read | fixed | open |
| EXT-89 | `sys/dev/usb/usb_subr.c:533` | short interface/endpoint descriptors accepted | device | memory corruption | read | fixed | open |
| EXT-96 | `sys/dev/usb/uaudio.c:4432` | more than 8 channels written past `level[8]` | device | memory corruption | read | fixed | open |
| EXT-100 | `sys/dev/usb/uhidev.c:530` | zero-length report wraps the length | device | memory corruption | read | fixed | open |
| EXT-177 | `sys/lib/libsa/alloc.c:185` | arm64 efiboot heap has no limit | device | memory corruption | read | fixed | open |
| EXT-178 | `sys/arch/arm64/stand/efiboot/efidev.c:106` | whole sectors copied past the caller's size (also amd64) | device | memory corruption | read | fixed | open |
| EXT-200 | `sys/dev/ic/ncr53c9x.c:1341` | a reselection with no target bit indexes `sc_tinfo[255]` | device | memory corruption | read | not ported (M17) | open |
| EXT-32 | `sys/ufs/ext2fs/ext2fs_bmap.c:110` | `ext4_bmapext` uses an uninitialised extent path | device | crash | read | fixed | open |
| EXT-34 | `sys/ufs/ext2fs/ext2fs_extents.c:73` | extent binary search trusts `eh_ecount` | device | crash | read | fixed | open |
| EXT-35 | `sys/ufs/ext2fs/ext2fs_lookup.c:175` | `ext2fs_readdir` reads past its buffer | device | crash | read | fixed | open |
| EXT-36 | `sys/ufs/ext2fs/ext2fs_lookup.c:638` | `ext2fs_search_dirblock` loops on a zero reclen | device | crash | read | fixed | open |
| EXT-45 | `sys/ntfs/ntfs_compr.c:78` | LZNT1 back reference reads before the output | device | crash | read | fixed | open |
| EXT-47 | `sys/ntfs/ntfs_subr.c:336` | MFT attribute walk unbounded | device | crash | read | fixed | open |
| EXT-48 | `sys/ntfs/ntfs_subr.c:207` | attribute-list walk loops or hits NULL | device | crash | read | fixed | open |
| EXT-50 | `sys/ntfs/ntfs_subr.c:893` | index-entry walks loop on reclen 0 | device | crash | read | fixed | open |
| EXT-52 | `sys/ntfs/ntfs_vfsops.c:312` | divide by a zero sector size | device | crash | read | fixed | open |
| EXT-59 | `sys/isofs/udf/udf_vnops.c:468` | `udf_read` spins on an empty embedded file | device | crash | read | fixed | open |
| EXT-60 | `sys/isofs/udf/udf_vnops.c:659` | negative FID fragment size copied | device | crash | read | fixed | open |
| EXT-64 | `sys/isofs/udf/udf_vfsops.c:618` | file entry size from disk copied from one block | device | crash | read | fixed | open |
| EXT-65 | `sys/isofs/udf/udf_subr.c:264` | `udf_vat_read` calls `brelse(NULL)` | device | crash | read | fixed | open |
| EXT-67 | `sys/isofs/udf/udf_vfsops.c:794` | partition map and sparing table read past their buffers | device | crash | read | fixed | open |
| EXT-70 | `sys/msdosfs/msdosfs_fat.c:917` | `fillinusemap` calls `brelse(NULL)` | device | crash | read | fixed | open |
| EXT-75 | `sys/scsi/scsi_base.c:1224` | MODE SENSE/SELECT(10) trust the device's lengths | device | crash | read | fixed | open |
| EXT-88 | `sys/dev/usb/usb_subr.c:667` | config descriptor's `wTotalLength` trusted after the read | device | crash | read | fixed | open |
| EXT-90 | `sys/dev/usb/usbdi.c:707` | `usbd_get_no_alts` loops on a zero-length descriptor | device | crash | read | fixed | open |
| EXT-91 | `sys/dev/usb/usbdi_util.c:218` | `usbd_get_hid_descriptor` loops on a zero-length descriptor | device | crash | read | fixed | open |
| EXT-92 | `sys/dev/usb/ohci.c:525` | divide by a zero `wMaxPacketSize` | device | crash | read | panics instead | open |
| EXT-95 | `sys/dev/usb/uaudio.c:1029` | UAC 2.0 clock chain ending in NULL | device | crash | read | fixed | open |
| EXT-97 | `sys/dev/usb/uaudio.c:1170` | unnamed features reach `strcmp(NULL)` | device | crash | read | fixed | open |
| EXT-99 | `sys/dev/usb/uaudio.c:2045` | unknown class version leaves locals uninitialised | device | crash | read | fixed | open |
| EXT-104 | `sys/dev/usb/uftdi.c:949` | short packet wraps the count | device | crash | read | fixed | open |
| EXT-106 | `sys/dev/hid/hidkbd.c:737` | NULL `sc_var` on allocation failure; uninitialised entries | device | crash | read | fixed | open |
| EXT-108 | `sys/dev/hid/hidms.c:584` | `INT_MIN / -1` in the calibration | device | crash | read | same fault (see below) | open |
| EXT-33 | `sys/ufs/ext2fs/ext2fs_bmap.c:111` | ext4 extent lookups leave the leaf buffer busy | device | leak | read | fixed | open |
| EXT-39 | `sys/miscfs/fuse/fuse_vnops.c:881` | `fusefs_readdir` leaks the fusebuf | device | leak | read | fixed | open |
| EXT-40 | `sys/miscfs/fuse/fuse_vnops.c:1532` | `fusefs_mkdir` leaks the fusebuf | device | leak | read | fixed | open |
| EXT-42 | `sys/miscfs/fuse/fuse_device.c:120` | cleared FORGET and INIT fusebufs never freed | device | leak | read | same fault (see below) | open |
| EXT-44 | `sys/miscfs/fuse/fuse_lookup.c:185` | `..` not a directory leaks the locked vnode | device | leak | read | same fault (see below) | open |
| EXT-54 | `sys/ntfs/ntfs_vfsops.c:388` | `$AttrDef` vnode kept locked on the error path | device | leak | read | fixed | open |
| EXT-56 | `sys/isofs/cd9660/cd9660_vfsops.c:334` | `iso_mountfs` leaks the primary descriptor, mount hangs | device | leak | read | fixed | open |
| EXT-68 | `sys/isofs/udf/udf_vfsops.c:433` | `udf_mountfs` error path leaks the tables | device | leak | read | fixed | open |
| EXT-74 | `sys/scsi/cd.c:1990` | `dvd_read_bca` leaks its buffer | device | leak | read | fixed | open |
| EXT-62 | `sys/isofs/udf/udf_vnops.c:301` | month past 12 reads past `mon_lens` | device | wrong result | read | fixed | open |
| EXT-63 | `sys/isofs/udf/udf_vnops.c:1328` | one allocation descriptor read past `l_ad` | device | wrong result | read | fixed | open |
| EXT-83 | `sys/dev/softraid_crypto.c:897` | old-format mask key read at the wrong offset | device | wrong result | read | same fault (see below) | open |
| EXT-101 | `sys/dev/usb/uhidev.c:819` | `USBD_IN_PROGRESS` taken as failure | device | wrong result | read | same fault (see below) | open |
| EXT-102 | `sys/dev/usb/uhub.c:324` | port bits above 31 shifted out of an int | device | wrong result | read | fixed | open |
| EXT-107 | `sys/dev/hid/hidms.c:233` | Wacom pad buttons copied from the wrong index | device | wrong result | read | same fault (see below) | open |
| EXT-176 | `sys/lib/libsa/cd9660.c:240` | directory records walked past the block | device | wrong result | read | fixed | open |
| EXT-216 | `sys/dev/ic/ncr53c9x.c:1454` | message-in reads `sc_imess` past its 9 bytes | device | wrong result | read | not ported (M17) | open |
| EXT-76 | `sys/scsi/sd.c:1374` | `viscpy` reads past the INQUIRY field | device | cosmetic | read | fixed | open |
| EXT-98 | `sys/dev/usb/uaudio.c:726` | name list keeps a dead stack buffer | device | cosmetic | read | fixed | open |
| EXT-41 | `sys/miscfs/fuse/fuse_device.c:136` | `fuse_device_cleanup` corrupts the queues | root | memory corruption | read | fixed | open |
| EXT-116 | `sys/dev/ata/atascsi.c:1044` | UNMAP completes the xfer and carries on | root | memory corruption | read | fixed | open |
| EXT-117 | `sys/dev/ata/atascsi.c:1135` | unmap failure frees `xa->data`, not its buffer | root | memory corruption | read | fixed | open |
| EXT-130 | `sys/dev/ic/fxp.c:1847` | oversized microcode file copied past the block | root | memory corruption | read | fixed | open |
| EXT-175 | `sys/lib/libsa/tftp.c:169` | long path overflows the TFTP request buffer | root | memory corruption | read | fixed | open |
| EXT-201 | `sys/dev/ic/wdc.c:1490` | an odd-length PIO input writes one byte past the buffer | root | memory corruption | read | fixed | open |
| EXT-13 | `sys/kern/kern_unveil.c:266` | cover walk from a slot an unmount zapped (NULL vnode) | root | crash | read | fixed | open |
| EXT-14 | `sys/kern/kern_unveil.c:266` | cover walk spins outside a chroot | root | crash | read | fixed | open |
| EXT-84 | `sys/dev/softraid_concat.c:140` | block after the end reaches `sv_chunks[no_chunk]` | root | crash | read | fixed | open |
| EXT-118 | `sys/dev/ata/atascsi.c:1503` | REQUEST SENSE without data writes through NULL | root | crash | read | fixed | open |
| EXT-126 | `sys/dev/pci/ppb.c:785` | hot-plug with no pci bus behind the bridge | root | crash | read | fixed | open |
| EXT-154 | `sys/dev/acpi/acpimadt.c:370` | NULL I/O APIC for an override or ISA pin | root | crash | read | fixed | open |
| EXT-11 | `sys/net/pf_norm.c:1010` | `pf_refragment6` leaks the fragment list when forwarding goes off | root | leak | read | fixed | open |
| EXT-17 | `sys/net/hfsc.c:479` | queue stats copied out with uninitialised padding | root | leak | read | fixed | open |
| EXT-37 | `sys/ufs/ext2fs/ext2fs_vfsops.c:464` | `ext2fs_reload` keeps the superblock buffer busy | root | leak | read | fixed | open |
| EXT-82 | `sys/dev/softraid_crypto.c:784` | key-disk metadata with the mask key leaked unzeroed | root | leak | read | fixed | open |
| EXT-111 | `sys/dev/ic/nvme.c:1014` | passthrough copies out `pt_statuslen` stack bytes | root | leak | read | fixed | open |
| EXT-156 | `sys/dev/wscons/wsdisplay.c:3487` | copy buffer of unchanged size leaked | root | leak | read | fixed | open |
| EXT-16 | `sys/net/pf_ioctl.c:1481` | `0xffffffff << 32` for an IPv4 source-limit prefix of 0 | root | wrong result | read | fixed | open |
| EXT-21 | `sys/kern/subr_userconf.c:338` | UKC number parser accepts a digit equal to the base | root | wrong result | read | same fault (see below) | open |
| EXT-22 | `sys/kern/kern_watchdog.c:55` | `wdog_period * 1000` overflows int | root | wrong result | read | fixed | open |
| EXT-81 | `sys/dev/softraid_crypto.c:368` | KDF hint copied past `kdfinfo` | root | wrong result | read | fixed | open |
| EXT-155 | `sys/dev/wscons/wsemul_dumb.c:102` | `crippled` uninitialised for non-console screens | root | wrong result | read | fixed | open |
| EXT-157 | `sys/dev/wscons/wsdisplay.c:3441` | paste after reallocation reads stale memory | root | wrong result | read | fixed | open |
| EXT-173 | `sys/arch/arm64/arm64/db_interface.c:90` | ddb's `$x30` is past the trapframe | root | wrong result | read | fixed | open |
| EXT-217 | `sys/dev/ic/mpi.c:3060` | the cache ioctl reads its reply from the rcb, not the reply frame | root | wrong result | read | faithful | open |
| EXT-218 | `sys/dev/ata/wd.c:905` | a failed dump leaves `wddoingadump` set; later dumps get EFAULT | root | wrong result | read | faithful | open |
| EXT-219 | `etc/etc.amd64/MAKEDEV:606` | fd1's nodes get unit 0's minors (U*128, FDUNIT divides by 512) | root | wrong result | read | not used (only fd0 in the ramdisk) | open |
| EXT-18 | `sys/kern/kern_descrip.c:116` | `find_next_zero` scans past the fd bitmaps | root | cosmetic | read | fixed | open |
| EXT-19 | `sys/net/pfkeyv2.c:1939` | ADDFLOW copies a whole sockaddr_union from a sockaddr_in | root | cosmetic | read | fixed | open |
| EXT-69 | `sys/kern/vfs_init.c:91` | UDF's `vfc_datasize` is `struct iso_args`'s | root | cosmetic | read | fixed | open |
| EXT-161 | `sys/dev/audio.c:1282` | uninitialised mixer entries saved and restored | root | cosmetic | read | fixed | open |
| EXT-167 | `sys/dev/gpio/gpio.c:419` | GPIOATTACH prints through the wrong print function | root | cosmetic | read | fixed | open |
| EXT-220 | `sys/dev/ic/mpi.c:3297` | the volume ioctl copies 32 bytes from an 8-byte vendor field | root | cosmetic | read | faithful | open |
| EXT-221 | `sys/dev/ic/mpi.c:3348` | the disk ioctl `strlcpy`s an unterminated 8-byte vendor id | root | cosmetic | read | faithful | open |
| EXT-77 | `sys/dev/softraid_raid5.c:486` | deferred RAID 5/6 write queued twice after a read error | firmware/hardware | memory corruption | read | fixed | open |
| EXT-94 | `sys/dev/usb/xhci.c:839` | slot, endpoint and port ids from TRBs index arrays | firmware/hardware | memory corruption | read | fixed | open |
| EXT-109 | `sys/dev/ic/nvme.c:1218` | completion's command id indexes `sc_ccbs` unchecked | firmware/hardware | memory corruption | read | panics instead | open |
| EXT-110 | `sys/dev/ic/nvme.c:1148` | late completion writes through the restored cookie | firmware/hardware | memory corruption | read | fixed | open |
| EXT-115 | `sys/dev/ic/nvme.c:1390` | `sc_q` dangles after a failed resume | firmware/hardware | memory corruption | read | fixed | open |
| EXT-119 | `sys/dev/ic/siop.c:374` | DSA and reselection tag index arrays unchecked | firmware/hardware | memory corruption | read | fixed | open |
| EXT-120 | `sys/dev/ic/ahci.c:2263` | slot numbers from the controller and disk index `ap_ccbs` | firmware/hardware | memory corruption | read | panics instead | open |
| EXT-121 | `sys/dev/pci/virtio_pci.c:478` | capability BAR number indexes a 6-entry array | firmware/hardware | memory corruption | read | fixed | open |
| EXT-127 | `sys/dev/pci/uhci_pci.c:222` | stale `sc_ih` disestablished twice (also re_pci) | firmware/hardware | memory corruption | read | fixed | open |
| EXT-131 | `sys/dev/pci/if_dc_pci.c:513` | softc allocated with `struct dc_softc`'s size | firmware/hardware | memory corruption | read | fixed | open |
| EXT-136 | `sys/dev/ic/dc.c:1990` | PNIC workaround overruns its salvage buffer | firmware/hardware | memory corruption | read | fixed | open |
| EXT-141 | `sys/dev/acpi/dsdt.c:4117` | `Buffer(n){init}` copies the whole initializer | firmware/hardware | memory corruption | read | fixed | open |
| EXT-142 | `sys/dev/acpi/dsdt.c:2925` | buffer fields not bounds-checked | firmware/hardware | memory corruption | read | fixed | open |
| EXT-149 | `sys/dev/acpi/acpicpu_x86.c:1212` | C4 entry increments past `cst_stats[4]` | firmware/hardware | memory corruption | read | fixed | open |
| EXT-153 | `sys/dev/acpi/acpiprt.c:380` | bus number over 255 writes past `mp_busses` | firmware/hardware | memory corruption | read | fixed | open |
| EXT-168 | `sys/dev/ofw/ofw_misc.c:1103` | more than 2 IOMMU cells overrun `cells[2]` | firmware/hardware | memory corruption | read | fixed | open |
| EXT-170 | `sys/dev/ipmi.c:557` | BT/SMIC receive trusts the BMC's length | firmware/hardware | memory corruption | read | fixed | open |
| EXT-198 | `sys/dev/ic/mpi.c:1230` | a timed-out `mpi_poll` leaves the reply pointing at its dead stack frame | firmware/hardware | memory corruption | read | fixed | open |
| EXT-199 | `sys/dev/pci/pcscp.c:466` | the leftover FIFO byte is stored before the negative-count check | firmware/hardware | memory corruption | read | not ported (M17) | open |
| EXT-114 | `sys/dev/ic/nvme.c:2131` | `rp` indexes a 4-entry array | firmware/hardware | crash | read | fixed | open |
| EXT-122 | `sys/dev/pv/vioblk.c:344` | completion for an empty slot dereferences NULL | firmware/hardware | crash | read | panics instead | open |
| EXT-128 | `sys/dev/pci/ahci_pci.c:439` | NULL interrupt handle disestablished | firmware/hardware | crash | read | fixed | open |
| EXT-143 | `sys/dev/acpi/dsdt.c:3986` | AML parser reads past the table | firmware/hardware | crash | read | fixed | open |
| EXT-145 | `sys/dev/acpi/acpi.c:1189` | no DSDT dereferences NULL | firmware/hardware | crash | read | fixed | open |
| EXT-147 | `sys/dev/acpi/acpi.c:1544` | GAS access-size code used as a byte count | firmware/hardware | crash | read | same fault (see below) | open |
| EXT-148 | `sys/dev/acpi/acpicpu_x86.c:1107` | divide by zero and negative `_PSS` index | firmware/hardware | crash | read | fixed | open |
| EXT-150 | `sys/dev/acpi/acpicpu_x86.c:965` | `_PSS`/`_PCT` used without type or length checks | firmware/hardware | crash | read | fixed | open |
| EXT-151 | `sys/dev/acpi/acpidmar.c:2782` | DMAR/IVRS walks loop on length 0 | firmware/hardware | crash | read | fixed | open |
| EXT-162 | `sys/dev/pci/auich.c:1469` | calibration divides by a zero interval | firmware/hardware | crash | read | fixed | open |
| EXT-171 | `sys/dev/ipmi.c:1576` | unknown interface type dereferences NULL | firmware/hardware | crash | read | fixed | open |
| EXT-172 | `sys/arch/arm64/dev/acpiiort.c:82` | IORT walks not bounded by the table | firmware/hardware | crash | read | fixed | open |
| EXT-202 | `sys/dev/sdmmc/sdmmc_io.c:373` | a card that cannot be selected releases the bus lock twice | firmware/hardware | crash | read | faithful | open |
| EXT-80 | `sys/dev/softraid_raid6.c:655` | `sr_raid6_intr` leaks a failed read's opaque | firmware/hardware | leak | read | fixed | open |
| EXT-134 | `sys/dev/ic/dc.c:3103` | detach unmaps one page of three | firmware/hardware | leak | read | fixed | open |
| EXT-138 | `sys/dev/acpi/dsdt.c:2164` | `aml_compare` leaks the converted operand | firmware/hardware | leak | read | fixed | open |
| EXT-139 | `sys/dev/acpi/dsdt.c:2982` | Create*Field leaks the converted buffer | firmware/hardware | leak | read | fixed | open |
| EXT-140 | `sys/dev/acpi/dsdt.c:1845` | Return inside While leaks scopes | firmware/hardware | leak | read | fixed | open |
| EXT-163 | `sys/dev/pci/azalia.c:2647` | deleting a codec leaks its connection lists | firmware/hardware | leak | read | fixed | open |
| EXT-205 | `sys/dev/ic/mpi.c:816` | a failed `mpi_poll` leaks its ccb (port enable, config pages, cache ioctl) | firmware/hardware | leak | read | faithful | open |
| EXT-20 | `sys/kern/clock_subr.c:104` | RTC month past 12 indexes past `month_days`; int overflow | firmware/hardware | wrong result | read | fixed | open |
| EXT-112 | `sys/dev/ic/nvme.c:1281` | `(1 << mdts) * (1 << mpsmin)` overflows | firmware/hardware | wrong result | read | fixed | open |
| EXT-123 | `sys/dev/pci/if_vmx.c:313` | uninitialised interrupt handle after a failed MSI-X map | firmware/hardware | wrong result | read | fixed | open |
| EXT-125 | `sys/dev/pci/if_em_hw.c:7781` | PHY workaround writes uninitialised values | firmware/hardware | wrong result | read | fixed | open |
| EXT-129 | `sys/dev/ic/re.c:1998` | ISR acknowledged after the moderation timer starts | firmware/hardware | wrong result | read | fixed | open |
| EXT-132 | `sys/dev/ic/dc.c:1609` | Conexant MAC read from the softc, not the SROM | firmware/hardware | wrong result | read | fixed | open |
| EXT-133 | `sys/dev/ic/dc.c:1582` | MAC offset read into half an uninitialised int | firmware/hardware | wrong result | read | fixed | open |
| EXT-135 | `sys/dev/ic/dc.c:1512` | 21143 SROM parser trusts the SROM's offsets | firmware/hardware | wrong result | read | fixed | open |
| EXT-144 | `sys/dev/acpi/dsdt.c:4019` | `Ones` is 255 on arm64 | firmware/hardware | wrong result | read | fixed | open |
| EXT-146 | `sys/dev/acpi/acpi.c:1276` | short FADT read past its copy | firmware/hardware | wrong result | read | fixed | open |
| EXT-152 | `sys/dev/acpi/acpiprt.c:249` | uninitialised IRQ without an interrupt descriptor | firmware/hardware | wrong result | read | fixed | open |
| EXT-164 | `sys/dev/pci/azalia.c:2344` | Connection Select index trusted | firmware/hardware | wrong result | read | fixed | open |
| EXT-166 | `sys/dev/pci/azalia.c:2926` | uninitialised digital-control value written | firmware/hardware | wrong result | read | fixed | open |
| EXT-169 | `sys/dev/ipmi.c:1139` | 6-bit sensor names read past the SDR | firmware/hardware | wrong result | read | fixed | open |
| EXT-179 | `sys/arch/arm64/stand/efiboot/fdt.c:169` | FDT walkers read past the blob | firmware/hardware | wrong result | read | fixed | open |
| EXT-183 | `sys/dev/pckbc/pckbd.c:685` | `pckbd_xtbl2_ext`'s first row has 15 entries, every later code one off | firmware/hardware | wrong result | read | faithful | open |
| EXT-187 | `sys/dev/pv/viomb.c:348` | a short deflate records one page fewer than it moved | firmware/hardware | wrong result | read | faithful | open |
| EXT-188 | `sys/dev/pv/viomb.c:320` | the page-number arrays the host reads are queued device-writable | firmware/hardware | wrong result | read | faithful | open |
| EXT-206 | `sys/dev/pci/pciide.c:4217` | CMD680 timings write both bytes to the same register | firmware/hardware | wrong result | read | faithful | open |
| EXT-207 | `sys/dev/pci/pciide.c:6088` | HPT's channel loop overwrites the compat channel taken from the function | firmware/hardware | wrong result | read | faithful | open |
| EXT-208 | `sys/dev/pci/pciide.c:5018` | CY82C693 stores DMA mode -1 as 255 in both drives | firmware/hardware | wrong result | read | faithful | open |
| EXT-209 | `sys/dev/pci/pciide.c:9023` | RDC setup clears the other channel's timings | firmware/hardware | wrong result | read | faithful | open |
| EXT-210 | `sys/dev/pci/pciide.c:1703` | compat unmap releases the control registers through the command handle | firmware/hardware | wrong result | read | faithful | open |
| EXT-211 | `sys/dev/pci/pciide.c:1787` | native-map error path unmaps the control base with the command tag | firmware/hardware | wrong result | read | faithful | open |
| EXT-212 | `sys/dev/sdmmc/sdmmc_mem.c:655` | a select or block-length failure is overwritten and lost | firmware/hardware | wrong result | read | faithful | open |
| EXT-213 | `sys/dev/sdmmc/sdmmc_mem.c:917` | an MMC bus-width failure is overwritten by the clock change | firmware/hardware | wrong result | read | faithful | open |
| EXT-214 | `sys/dev/ata/wd.c:1065` | `wd_flushcache` tests an error code as a command flag; its ENODEV never happens | firmware/hardware | wrong result | read | faithful | open |
| EXT-190 | `sys/dev/pv/viogpu.c:173` | the `softintr_establish` cookie is dropped and `viogpu_rx_soft` never scheduled | firmware/hardware | cosmetic | read | faithful | open |
| EXT-191 | `sys/dev/pv/viogpu.c:214` | the attach line lacks its newline before `virtio_attach_finish` prints | firmware/hardware | cosmetic | reproduced | faithful | open |
| EXT-222 | `sys/dev/ic/mpireg.h:754` | `mpi_msg_eventack_reply`'s `ioc_status` is 32 bits where every other reply has 16 | firmware/hardware | cosmetic | read | faithful | open |
| EXT-86 | `sys/dev/usb/ugen.c:393` | failed isochronous open frees started transfers | alloc-failure | memory corruption | read | same fault (see below) | open |
| EXT-105 | `sys/dev/usb/ucom.c:551` | open failure frees the HID device's transfer | alloc-failure | memory corruption | read | fixed | open |
| EXT-165 | `sys/dev/pci/azalia_codec.c:1276` | failed mixer growth ignored, array overrun | alloc-failure | memory corruption | read | fixed | open |
| EXT-204 | `sys/dev/ic/ufshci.c:1429` | a failed ccb allocation frees the request list the running controller uses | alloc-failure | memory corruption | read | not ported (M17) | open |
| EXT-12 | `sys/net/if_wg.c:640` | `art_insert` failure dereferenced as a node | alloc-failure | crash | read | fixed | open |
| EXT-55 | `sys/ntfs/ntfs_vfsops.c:759` | `ntfs_vgetex` releases the ntnode twice | alloc-failure | crash | read | same fault (see below) | open |
| EXT-93 | `sys/dev/usb/ohci.c:2761` | bulk start uses the tail before the error check | alloc-failure | crash | read | fixed | open |
| EXT-113 | `sys/dev/ic/nvme.c:1425` | PRP-list allocation used unchecked | alloc-failure | crash | read | fixed | open |
| EXT-124 | `sys/dev/pci/if_vmx.c:569` | RSS DMA allocation used unchecked | alloc-failure | crash | read | fixed | open |
| EXT-174 | `sys/dev/fdt/pciecam.c:223` | NULL extent passed to `extent_free` | alloc-failure | crash | read | fixed | open |
| EXT-189 | `sys/dev/pv/viogpu.c:296` | attach's error paths unmap the address of the pointer, not the mapping | alloc-failure | crash | read | fixed | open |
| EXT-203 | `sys/dev/pci/pciide.c:4286` | six chip maps use their `M_NOWAIT` cookie unchecked | alloc-failure | crash | read | panics | open |
| EXT-78 | `sys/dev/softraid_raid5.c:608` | RAID 5 leaks a strip block on error | alloc-failure | leak | read | fixed | open |
| EXT-79 | `sys/dev/softraid_raid6.c:632` | RAID 6 leaks blocks and ccbs on error | alloc-failure | leak | read | fixed | open |
| EXT-215 | `sys/dev/pci/pcscp.c:304` | attach's error path unmaps the DMA map pointer, not the MDL mapping | alloc-failure | wrong result | read | not ported (M17) | open |

## Entries (EXT-1 to EXT-8, EXT-192 to EXT-197)

### EXT-1: EDK2 UhciDxe ASSERT on arm64 (QEMU firmware)

- Where: the EDK2 build QEMU 11.1.2 ships for `virt` (`edk2-aarch64-code.fd`),
  `MdeModulePkg/Bus/Pci/UhciDxe/UhciSched.c` line 974.
- What: with `piix3-usb-uhci` on the PCI bus, about one boot in five stops in the firmware,
  before any OS loader runs: `ASSERT [UhciDxe] /home/kraxel/projects/qemu/roms/edk2/
  MdeModulePkg/Bus/Pci/UhciDxe/UhciSched.c(974): CR has Bad Signature`. A `CR()` signature
  check failing means the driver followed a pointer to a structure that is not the one it
  expects (freed or never initialised): a firmware bug, independent of the OS.
- Seen: `smoke-uhci`'s arm64 boot (M16b, 2026-10-08; again in `just ci` on 2026-10-09).
- Port's handling: `xtask smoke` boots once more when a failed boot's transcript has that
  exact line and no kernel line yet (`boot::FIRMWARE_FLAKES`, the user's decision of
  2026-10-09); the log says `retrying once after a known firmware bug (EXT-1 ...)` and
  `smoke-all` counts such retries. A second failure of any kind fails the smoke.
- To report: QEMU's EDK2 build (or EDK2's `UhciDxe`), with the QEMU command line of
  `smoke-uhci`'s arm64 boot and the frequency. Remove the retry once fixed.

### EXT-2: xtask frozen before main on macOS (host)

- What: an `xtask` process that `cargo` has just started prints nothing and never ends;
  sampled once in `_dyld_start`, macOS's dynamic loader, before `main`. Any limit inside it
  is never reached.
- Seen: `smoke-softraid` in a `ci-full` run (2026-10-09, 2 h 30 min until noticed);
  `smoke-pcn` in `just ci` (2026-10-09 13:35, stopped by the watchdog after 10 minutes). Two
  in some 300 recipe runs.
- Cause: unknown. Candidates: the loader waiting on the system's code-signing or malware
  assessment of a freshly linked binary while many processes start at once.
- Port's handling: `smoke-all`'s outer watchdog (`TIMEOUT`); since 3bf642a1 it writes the
  stuck processes' state and wait channel to the log, which is what the next case needs.
- To report: Apple (Feedback Assistant), once the listing shows where it waits.

### EXT-3: QEMU TCG loses `sev` on arm64

- What: under MTTCG with two or more vCPUs, QEMU's `sev` helper kicks a halted vCPU without
  the global lock, and the event can be lost; a CPU in `wfe` waiting for it never wakes. An
  application processor waiting for `CPUF_GO`, its GIC CPU interface not enabled yet, then
  hangs the boot.
- Seen: M11e, the arm64 MP boot.
- Port's handling: arm64 turns on the generic timer's event stream (`CNTKCTL_EL1.EVNTEN`, a
  wake-up about every 130 us) on every CPU, which OpenBSD does not (docs/ARCHITECTURE.md,
  "Deviations"); Linux keeps it on for the same reason.
- To report: QEMU (target/arm, the `sev`/`wfe` TCG helpers). The user's decision, pending
  since M11 (docs/STATUS.md).

### EXT-4: ehci_pci writes EHCI_USBINTR with a 16-bit access (OpenBSD)

- Where: `sys/dev/pci/ehci_pci.c:128` at 3ce1f3f79392: `EOWRITE2(&sc->sc, EHCI_USBINTR, 0);`.
  `EHCI_USBINTR` (`sys/dev/usb/ehcireg.h:124`, offset 0x08) is a 32-bit operational register;
  the EHCI specification (section 2.3) asks for 32-bit accesses to them.
- What: QEMU's EHCI refuses the 2-byte access; on arm64 `virt` that is a synchronous
  external abort, so OpenBSD 8.0 panics at attach (the snapshot `diff-openbsd` uses, probe on
  the same QEMU setup, M16b). amd64's port I/O takes it.
- Verified: reproduced on OpenBSD 8.0 and on EmiBSD (the port keeps the 16-bit write).
- Port's handling: faithful; M16b's criterion for ehci was restated as "behaves as OpenBSD
  8.0" (docs/ROADMAP.md, M16b row; docs/ARCHITECTURE.md, ehci(4)).
- To report (after checking -current): OpenBSD bugs@, with the line, the spec reference, the
  arm64 panic and the one-line fix (`EOWRITE4`).

### EXT-5: ehci's INTx lost while cold on amd64 (QEMU I/O APIC and OpenBSD's routing)

- What: `ehci_init` raises INTx while the I/O APIC pin is still masked and edge-triggered
  (`ioapic_addroute` leaves pins to `ioapic_enable` while cold, as in the C), and QEMU's I/O
  APIC drops edges that arrive masked, so the level that never falls is never delivered.
  OpenBSD 8.0 prints `uhub0: device problem, disabling port 1` and the stick is not mounted.
- Verified: reproduced on OpenBSD 8.0 (M16b probe) and EmiBSD.
- To analyse: whether real I/O APICs latch it (a QEMU difference) or OpenBSD should not
  unmask a level source as an edge; then report to the side that is wrong.

### EXT-6: a write through ohci halts the controller (QEMU pci-ohci or ohci(4))

- What: reading the stick through `pci-ohci` works; the first write ends with
  `ohci0: unrecoverable error, controller halted`, on OpenBSD 8.0 as on EmiBSD (M16b).
- To analyse: which side breaks the OHCI transfer descriptors' contract.

### EXT-7: tulip passes no traffic (QEMU tulip or dc(4))

- What: OpenBSD 8.0 on QEMU's `tulip` (DEC 21143): `dc0 at pci0 dev 2 function 0
  "DEC 21142/3" rev 0x00 ...`, `lxtphy0 at dc0 phy 1: LXT970, rev. 0`, `dc0: failed to force
  tx to idle state` (twice), `dc0: watchdog timeout`, `3 packets transmitted, 0 packets
  received, 100.0% packet loss` (M16c probe). EmiBSD's dc(4) does the same (`smoke-dc`).
- To analyse: QEMU's tulip model (the transmit state machine) against the 21143 manual.

### EXT-8: igb receives nothing (QEMU igb or em(4))

- What: OpenBSD 8.0 on QEMU's `igb` (82576): `em0 at pci0 dev 2 function 0 "Intel 82576"
  rev 0x01: msi, address 52:54:00:12:34:56` (`dev 1` on arm64), status active,
  `3 packets transmitted, 0 packets received, 100.0% packet loss`, both archs (M16c probe).
  EmiBSD's em(4) does the same (`smoke-igb`). em(4) drives the 82576 with legacy
  descriptors.
- To analyse: whether QEMU's igb implements the legacy descriptor format the 82576 still
  supports.

### EXT-192: megasas and megasas-gen2 fail firmware initialisation (QEMU megasas or mfi(4))

- What: OpenBSD 8.0 (the diff-openbsd snapshot) on QEMU 11.1.2 with `-device megasas` (LSI
  MegaRAID SAS 1078) or `-device megasas-gen2` (SAS2108, product 0x0079, which mfi(4)'s table
  takes; QEMU has no MegaRAID Fusion device for mfii(4)), a `scsi-hd` at target 0
  (`cargo xtask diff-openbsd --arch A --megasas FILE probe`, `--megasas-gen2`, M16a):
  `mfi0 at pci0 dev 4 function 0 "Symbios Logic SAS1078" rev 0x00: apic 0 int 20` (gen2:
  `"Symbios Logic MegaRAID SAS2108 GEN2"`; arm64 `dev 1 ... : irq`), then
  `mfi0: could not initialize firmware` and `mfi0: can't attach` on both archs and both
  models. The message is `mfi_initialize_firmware`'s failure
  (`sys/dev/ic/mfi.c:754` at 3ce1f3f79392).
- Port: mfi(4) and mfii(4) are not ported; moved to M17 (real hardware) by the user on
  2026-10-09.
- To analyse: what the MFI_CMD_INIT frame carries and what QEMU's megasas expects of it.

### EXT-193: am53c974 and dc390 return an empty INQUIRY (QEMU am53c974 or pcscp(4))

- What: OpenBSD 8.0 on QEMU 11.1.2 with `-device am53c974` (or `dc390`) and a `scsi-hd` at
  target 0 (`diff-openbsd --arch amd64 --am53c974 FILE probe`, M16a):
  `pcscp0 at pci0 dev 4 function 0 "AMD 53c974 PCscsi-PCI" rev 0x10: apic 0 int 20`,
  `pcscp0: AM53C974, 40MHz`, `scsibus2 at pcscp0: 8 targets, initiator 7`,
  `probe(pcscp0:0:0): Check Condition (error 0) on opcode 0x0`,
  `sd1 at scsibus2 targ 0 lun 0: <, , >`, then `fdisk: opendev('sd1', 0x2): Device not
  configured`.
- Port: ncr53c9x and pcscp are not ported; moved to M17 by the user on 2026-10-09 (an agent's
  partial port is kept on its branch for M17).
- To analyse: the ESP/DMA handshake of QEMU's model against ncr53c9x.c's data-in phase. The
  porting agent's reading (claimed, not checked against QEMU's source, which is not on the
  machine): pcscp(4) runs every data-in phase in the DMA engine's MDL mode, writing only the
  page offset to `DMA_SPA` and the pages to the MDL; if QEMU's `esp-pci` does not implement
  MDL mode, the data goes to guest physical page 0, which matches the zero sense data and the
  empty INQUIRY.

### EXT-194: ufs hangs the boot after the LU attaches (QEMU ufs or ufshci(4))

- What: OpenBSD 8.0 amd64 on QEMU 11.1.2 with `-device ufs` and one `ufs-lu` (4096-byte
  logical blocks; `diff-openbsd --arch amd64 --ufs FILE probe`, M16a):
  `ufshci0 at pci0 dev 4 function 0 vendor "Red Hat", unknown product 0x0013 rev 0x00: apic 0
  int 20, UFSHCI 4.10`, `scsibus2 at ufshci0: 2 targets, initiator 0`,
  `sd1 at scsibus2 targ 1 lun 0: <QEMU, QEMU HARDDISK, 2.5+>`,
  `sd1: 64MB, 4096 bytes/sector, 16384 sectors`; the rest of autoconf attaches (up to
  `scsibus5 at softraid0: 256 targets`) and then nothing: no `root on sd0a`, no `login:` in
  900 s, one vCPU at 100 %. arm64's GENERIC has ufshci only at acpi and fdt: the PCI function
  is `not configured` there.
- Port: ufshci is not ported; moved to M17 by the user on 2026-10-09 (an agent's partial port
  is kept on its branch for M17).
- To analyse: which I/O never completes. An agent's partial port (M17 branch) hangs EmiBSD at
  the same place with the same attach lines; instrumented, no READ reached
  `ufshci_scsi_io` and no interrupt came before the hang, so the stall is earlier (an
  INQUIRY or capacity command, or `sdopen`) than softraid's metadata read.

### EXT-195: pvscsi has no configuration command (QEMU pvscsi or vmwpvs(4))

- What: OpenBSD 8.0 amd64 on QEMU 11.1.2 with `-device pvscsi` and a `scsi-hd`
  (`diff-openbsd --arch amd64 --pvscsi FILE probe`, M16a):
  `vmwpvs0 at pci0 dev 4 function 0 "VMware PVSCSI" rev 0x02: msi`,
  `vmwpvs0: get configuration failed`, and no scsibus. `vmwpvs_get_config`
  (`sys/dev/pci/vmwpvs.c`, VMWPVS_CMD_CONFIG) preloads the page header with INVPARAM/CHECK and
  reads it back unchanged: QEMU's pvscsi does not seem to implement that command.
- Port: vmwpvs(4) is ported whole and behaves the same (`smoke-vmwpvs` expects
  `vmwpvs0: get configuration failed`); M16a's criterion restated by the user.
- To analyse: QEMU's pvscsi command set against VMware's; whether vmwpvs could live without
  the configuration page (OpenBSD's decision, not the port's).

### EXT-196: a floppy read that ends at a cylinder's end fails (QEMU isa-fdc or fd(4))

- What: on QEMU 11.1.2 (q35 with `-device isa-fdc`, a 1.44 MB image in drive B, M16a's
  `--floppy`), `dd if=/dev/rfd0c of=/dev/null bs=18k count=10` prints `fd0c: hard error
  reading fsbn 32 of 0-35 (st0 21<seek_cmplt> st1 0 st2 0 cyl 1 head 0 sec 1)` and `dd:
  /dev/rfd0c: Input/output error` on OpenBSD 8.0 (`diff-openbsd probe --floppy`) as on EmiBSD
  (`smoke-fd`). The read of head 1, sectors 15 to 18 of cylinder 0 (multi-track) ends at the
  cylinder's end; QEMU reports st0 0x21 (SEEK END and drive 1) with C/H/R 1/0/1 where a 765
  reports normal termination, and fd(4)'s completion check (`(st0 & 0xf8) != 0`,
  `sys/dev/isa/fd.c:843` at 3ce1f3f79392) rejects it, retries and gives up. mount_msdos(8)
  reads the image.
- Port: faithful; `smoke-fd` expects OpenBSD's lines, the hard error included.
- To analyse: QEMU's `fdctrl` result phase after a multi-track transfer against the 82077AA
  datasheet.

### EXT-197: drive 0 is never found on QEMU's floppy controller (QEMU isa-fdc and fd(4))

- What: with the image in drive A, OpenBSD 8.0 prints `fd0 at fdc0 drive 1: density unknown`
  and fd0 is not configured, on `pc` and `q35` (M16a probe). The porting agent's debug print
  (not committed) showed drive 0's SENSE INTERRUPT after RECALIBRATE answering st0 0xe0 (the
  "ready changed" code fdcprobe's controller reset leaves, `sys/dev/isa/fdc.c:116`, with SEEK
  END) and drive 1's 0x21: fdprobe never drains the four statuses a reset leaves (fdintr's
  reset completion does, `sys/dev/isa/fd.c:885`), and QEMU merges the pending one into the
  recalibrate's, so fdprobe's check (`sys/dev/isa/fd.c:233`) fails for the first drive.
- Port: faithful; `storage.rs` puts the image in drive B (unit 1), where both systems attach
  `fd0 at fdc0 drive 1: 1.44MB 80 cyl, 2 head, 18 sec` (an xtask option, not a deviation).
- To analyse: whether a real 765 also reports the reset's status there, which would make it
  fd.c's alone.

## OpenBSD C slips

Slips in OpenBSD's C at the pin that the ports fixed, bounded or kept. These include
out-of-bounds reads and writes, divisions by zero, NULL or freed-pointer uses, wrong sizes,
leaks and uninitialised values. The review was done on 2026-10-09, read-only, against
3ce1f3f79392. Its sources were every file's `## Deviations`, the `ports.toml` notes, and a
text sweep of `sys/`, `tools/xtask/src` and `docs/ARCHITECTURE.md`. Each candidate's C was
read with its callers to decide whether the fault can happen in OpenBSD as it is.

- **Looked at:** 346 candidate rows (some rows group several notes of one file).
- **Entries:** 172 (EXT-9 to EXT-180). 171 are `read` and 1 is `claimed` (EXT-23, which needs
  a producer of TSO packets with split headers). Every crash and memory-corruption entry was
  checked again against the C lines it quotes. None was dropped or downgraded in that check;
  two line numbers were corrected.
  - By severity: 42 memory corruption, 54 crash (hangs included), 32 leak, 34 wrong result,
    10 cosmetic.
  - By reach: 13 remote, 13 user, 56 device, 29 root, 50 firmware/hardware, 11 alloc-failure.

  | Reach | memory corruption | crash | leak | wrong result | cosmetic |
  |---|---|---|---|---|---|
  | remote | 3 | 1 | 8 | 0 | 1 |
  | user | 2 | 3 | 1 | 5 | 2 |
  | device | 12 | 26 | 9 | 7 | 2 |
  | root | 5 | 6 | 6 | 7 | 5 |
  | firmware/hardware | 17 | 12 | 6 | 15 | 0 |
  | alloc-failure | 3 | 6 | 2 | 0 | 0 |

- **Discarded:** 174 rows.
  - 75 `design`: a pointer or invariant the design guarantees (an attached driver has its
    softc, a TCP socket its tcpcb, every caller honours the contract).
  - 60 `not-a-fault`: the C is right and the port's note overstates it.
  - 13 `idiom`: a value never used, or harmless wrapping or shift arithmetic.
  - 12 `rust-artifact`: a check only Rust needs (e.g. "short arguments" where `sys_mount`
    copies in a fixed-size struct).
  - 9 `duplicate`.
  - 3 `same-panic`: GENERIC's `DIAGNOSTIC` panics there as the port does.
  - 2 `intentional`: amd64 `cpu_reset`'s divide, zlib's uninitialised window.
  - Among the M16b/M16c reports, four were discarded:
    - the cdce frame: the MTU keeps it within the buffer;
    - `ne2000_readmem`: every caller leaves room for the rounded byte;
    - `inphyattach`: attach runs only after match found the PHY;
    - the ASIX filter word: it reads arpcom's own `ac__pad`.
- **To report first** (each to be checked against -current, the rule above):
  - EXT-27, `nfsrv_create`'s double free. Any NFS client that can create a FIFO hits it when
    `VOP_MKNOD` fails. Fix: delete three lines.
  - EXT-57, `udf_readatoffset`. Any user reading a well-formed UDF file at an unaligned offset
    gets kernel memory. Fix: one line.
  - EXT-38, `fusefs_close` from `unp_gc`. Any user who can write on a FUSE mount can panic the
    kernel.
  - EXT-103, uftdi 2232H. `stty 0` on an FT2232H/FT4232H panics amd64. Fix: one line.
  - EXT-29, `nfs_lookitup`'s `vput(NULL)`. A failing NFS v3 server can crash the client.
    Fix: one line.
- **Seen in passing, not yet entries:** these are `claimed`, outside the port's notes, and
  traced only partly.
  - `nfs_vfsops.c:689-714` `mountnfs`: a failed root GETATTR leaves the root nfsnode on a
    freed mount.
  - `ext2fs_vfsops.c:357-374` `e2fs_sbfill`: a crafted group count makes `mallocarray` panic.
  - `softraid.c:864-866` `sr_meta_opt_load`: an unbounded `som_length` from disk.
  - `ntfs_subr.c:1255`: `bmp[blnum >> 3]` is not bounded by the bitmap.
  - `fuse_vnops.c:1213-1216` `fusefs_mknod`: the vnode is used after `vput`.
  - `ugen.c:921-944` `ugen_set_interface`: NULL endpoints after a failed `usbd_set_interface`.
  - `uaudio.c:966-1001` `uaudio_req_ranges`: leaks `req`, and truncates `wLength`.
  - `if_em_hw.c:5224-5231` `em_read_phy_reg`: leaks the SW/FW semaphore.
  - `if_vmx.c:1815-1823` `vmxnet3_dma_allocmem`: leaks on failure.
  - `ahci_pci.c:363-375`: the interrupt stays established after a failed attach.

### Bugs the port reproduces

These are places where EmiBSD's own code has the same fault as the C. They are queued for
after M16 and are not fixed here.

- EXT-86 ugen: `sys/dev/usb/ugen.rs:617-627`. The isochronous open's failure path frees
  transfers already started on the open pipe. Its `SAFETY` comment wrongly says "never
  started". The pipe and the ring also leak, as the Deviations line (`ugen.rs:82-84`) says.
- EXT-108 hidms: `sys/dev/hid/hidms.rs:678` (and 679). The calibration's plain `/` on `i32`
  panics on `i32::MIN / -1`; it needs `wrapping_div`, and the Deviations line (`hidms.rs:79`)
  claims the arithmetic wraps.
- EXT-101 uhidev: `sys/dev/usb/uhidev.rs:107` keeps the failure-path leak, and
  `UsbdStatus::is_err` (`sys/dev/usb/usbdi.rs:224`) counts `USBD_IN_PROGRESS` as an error. So
  `uhidev_set_report_async` clears the stall and reports -1 after every started transfer,
  and the Deviations do not say so.
- EXT-21 userconf: `sys/kern/subr_userconf.rs:85` keeps `userconf_number`'s off-by-one on
  purpose, and a test pins it (`subr_userconf.rs:1526`).
- EXT-159 rasops: `sys/dev/rasops/rasops8.rs:66-70` keeps `rasops8_putchar16`'s space loop,
  and with it the underline written two lines up, before the frame buffer on row 0.
- EXT-147 GAS: `sys/dev/acpi/acpi.rs:75-76` and `sys/dev/acpi/acpicpu_x86.rs:85-86` keep the
  Access Size code as a byte length (they only stop the overrun).
- EXT-30 nfsd: `sys/nfs/nfs_syscalls.rs:767-792` `nfsrv_zapsock` does not free `ns_frag`,
  and this is not in its Deviations.
- Kept as in C, and said so in the Deviations:
  - EXT-42: `sys/miscfs/fuse/fuse_device.rs:59-60`.
  - EXT-44: `sys/miscfs/fuse/fuse_lookup.rs:46-47`.
  - EXT-55: `sys/ntfs/ntfs_vfsops.rs:103`.
  - EXT-61: `sys/isofs/udf/udf_vnops.rs:74-76`.
  - EXT-83: `sys/dev/softraid_crypto.rs:89-92`, kept for on-disk compatibility.
  - EXT-107: `sys/dev/hid/hidms.rs:69-72`, the C's index on a zeroed array.
- Panics where the C corrupts or crashes (same outcome class, explicit):
  - EXT-46: `sys/ntfs/ntfs_compr.rs:170-174`, which its Deviations do not mention.
  - EXT-92: `sys/dev/usb/ohci.rs:100`.
  - EXT-109: `sys/dev/ic/nvme.rs:63-67`.
  - EXT-120: `sys/dev/ic/ahci.rs:80`.
  - EXT-122: `sys/dev/pv/vioblk.rs:98-99` and `sys/dev/pv/vioscsi.rs:65-66`.
- Doc fix to make: `sys/net/pf_ioctl.rs:95-97` says `DIOCIGETIFACES` copies out past the C's
  buffer. At the pin, `pfi_get_ifaces` stops at `n >= *size` (`pf_if.c:792-793`), so it
  does not. The note is stale.

## kern and net

Every C line in this and the following sections is at the pin 3ce1f3f79392, under
`reference/openbsd-src/`.

### EXT-9: IPsec-only forwarding leaks every dropped packet

- Where: `sys/netinet6/ip6_forward.c:318`, `ip6_forward`.
- What: the `IPV6_FORWARDING_IPSEC` drop does `error = EHOSTUNREACH; goto senderr;`
  (318-322), and nothing from `senderr:` (339) on frees `m`. The IPv4 twin goes through
  `bad`, which frees it (`ip_output.c:427-430`).
- Reach: remote. It needs `net.inet6.ip6.forwarding=2` on an IPSEC kernel; then every
  forwarded IPv6 packet that did not arrive over IPsec leaks a chain, and any host can send
  them.
- Verified: read.
- Port's handling: `sys/netinet6/ip6_forward.rs:66` frees it.
- Severity: leak (remote mbuf exhaustion).
- Fix: `m_freem(m);` before `goto senderr;`.

### EXT-10: `icmp6_redirect_input` leaks the packet when the interface is gone

- Where: `sys/netinet6/icmp6.c:1206`, `icmp6_redirect_input`.
- What: `ifp = if_get(m->m_pkthdr.ph_ifidx); if (ifp == NULL) return;` (1206-1208) returns
  without freeing `m`. Every other exit goes through `freeit`.
- Reach: remote. An on-link redirect has to arrive while its interface detaches;
  `if_detach` removes the index before it takes NET_LOCK (`if.c:1275`). It is a race.
- Verified: read.
- Port's handling: `sys/netinet6/icmp6.rs:105` frees it.
- Severity: leak.
- Fix: `goto freeit` instead of `return`.

### EXT-11: `pf_refragment6` leaks the fragment list when forwarding goes off

- Where: `sys/net/pf_norm.c:1010`, `pf_refragment6`.
- What: in the dequeue loop, `default: ip6stat_inc(ip6s_cantforward); return (PF_DROP);`
  (1010-1012). It frees neither `m` nor the rest of `ml`, and `*m0` is already NULL (993).
- Reach: root. Forwarded, reassembled IPv6 fragments must be in flight while root turns
  `net.inet6.ip6.forwarding` off. It is a race.
- Verified: read.
- Port's handling: `sys/net/pf_norm.rs:62` frees the list.
- Severity: leak.
- Fix: `m_freem(m); ml_purge(&ml);` before the return.

### EXT-12: `wg_aip_add` dereferences `art_insert`'s NULL

- Where: `sys/net/if_wg.c:640`, `wg_aip_add`.
- What: `art_insert` returns NULL when `art_table_get` cannot allocate (`art.c:455-458`).
  The `else` branch takes that as an existing node: `aip = (struct wg_aip *) node; if
  (aip->a_peer != peer)` (648-650). `rtable.c:583-586` handles the same NULL as ENOMEM.
- Reach: alloc-failure. Root adds allowed IPs (`ifconfig wgN wgpeer ... wgaip`) while the ART
  pools cannot allocate.
- Verified: read.
- Port's handling: `sys/net/if_wg.rs:106` returns ENOBUFS.
- Severity: crash.
- Fix: `if (node == NULL) { pool_put(&wg_aip_pool, aip); ret = ENOBUFS; }`.

### EXT-13: unveil re-walks a slot an unmount zapped

- Where: `sys/kern/kern_unveil.c:266`, `unveil_find_cover`, from `unveil_add_vnode`
  (386-388).
- What: `unveil_removevnode` sets `uv->uv_vp = NULL` (812) and keeps `uv_cover`.
  `unveil_add_vnode` then re-covers every slot with the same cover, passing
  `unveil_find_cover(pr->ps_uvpaths[i].uv_vp, p)`. That is NULL, and
  `while (vp != root && (vp->v_flag & VROOT))` (266) dereferences it.
- Reach: root. A process has unveiled a directory on a file system that root unmounts, or
  only tries to: `dounmount_leaf` zaps the unveils before `VFS_UNMOUNT`
  (`vfs_syscalls.c:488-489`). The process's next matching unveil(2) panics the kernel.
- Verified: read.
- Port's handling: `sys/kern/kern_unveil.rs:66` gives a zapped slot cover -1.
- Severity: crash.
- Fix: skip slots with `uv_vp == NULL` in that loop.

### EXT-14: unveil's cover walk spins outside a chroot

- Where: `sys/kern/kern_unveil.c:266`, `unveil_find_cover`.
- What: at `rootvnode`, `mnt_vnodecovered` is NULL, so `vp = vp->v_mount->mnt_vnodecovered ?
  ... : vp;` (269-270) does not move. When `root` is a chroot's `fd_rdir`, the loop never
  ends.
- Reach: root. Unveil a path outside, chroot(2), then unveil a path inside with the same
  cover.
- Verified: read.
- Port's handling: `sys/kern/kern_unveil.rs:63` ends the walk with -1.
- Severity: crash (a kernel hang).
- Fix: break with -1 when `mnt_vnodecovered` is NULL.

### EXT-15: a new session inherits a freed session's verauth values

- Where: `sys/kern/kern_proc.c:275`, `enternewpgrp`; the session comes from `pool_get`
  without `PR_ZERO` (`kern_prot.c:226`).
- What: the new-session branch (275-286) never sets `s_verauthuid`/`s_verauthppid`, and
  `SESSRELE` frees a session without clearing them. `TIOCCHKVERAUTH` grants when both match
  (`tty_tty.c:136-137`).
- Reach: user, contrived. The values survive when the verauth parent was in another
  session. A later setsid(2) that reuses the item, by a process with the same ruid and
  ppid, passes doas's persist check without a password.
- Verified: read.
- Port's handling: `sys/kern/kern_proc.rs:65` zeroes the session.
- Severity: wrong result (an authentication bypass in a narrow case).
- Fix: `zapverauth(newsess)` (or zero both fields) in the new-session branch.

### EXT-16: an IPv4 source-limit prefix of 0 shifts by 32

- Where: `sys/net/pf_ioctl.c:1481`, `pf_sourcelim_add`.
- What: `inet_prefix > 32` is refused (1444), but 0 is not. Then `htonl(0xffffffff << (32 -
  prefix))` (1481-1482) shifts by 32, which is undefined; amd64 and arm64 give all ones.
- Reach: root, through DIOCADDSOURCELIM directly (pfctl's parser refuses 0).
- Verified: read.
- Port's handling: `sys/net/pf_ioctl.rs:98` uses mask 0.
- Severity: wrong result.
- Fix: mask 0 for prefix 0, or reject 0.

### EXT-17: hfsc queue stats copied out with uninitialised padding

- Where: `sys/net/hfsc.c:479`, `hfsc_pf_qstats`.
- What: the stack `struct hfsc_class_stats stats;` (479) is never cleared, and
  `hfsc_getclstats` fills the members but not the 20 bytes of holes (LP64). `copyout(&stats,
  ubuf, sizeof(stats))` (500) sends them.
- Reach: root (DIOCGETQSTATS on an hfsc queue).
- Verified: read.
- Port's handling: `sys/net/hfsc.rs:97` zeroes it.
- Severity: leak (kernel stack bytes to userland).
- Fix: `memset(&stats, 0, sizeof(stats))`.

### EXT-18: `find_next_zero` scans past the fd bitmaps

- Where: `sys/kern/kern_descrip.c:116`, `find_next_zero`.
- What: `maxoff = NDLOSLOTS(bits)` (116) rounds up to 32 words, not to the array, so a full
  table of 25,600 descriptors reads `fd_himap[25..31]` and on into `fd_lomap`. The garbage
  index is rejected later (`i >= last`).
- Reach: root (limits above 25,600 descriptors).
- Verified: read.
- Port's handling: `sys/kern/kern_descrip.rs:85` reads past words as full.
- Severity: cosmetic (an out-of-bounds read whose result is discarded).
- Fix: bound `maxoff` by the bitmap's length.

### EXT-19: PF_KEY ADDFLOW copies a whole `sockaddr_union`

- Where: `sys/net/pfkeyv2.c:1939`, `pfkeyv2_dosend`.
- What: `bcopy(sunionp, &ipo->ipo_dst, sizeof(union sockaddr_union))` (1939-1941; `ssrc`
  1945-1947). An AF_INET extension holds 16 bytes, so this reads 12 bytes past it, past the
  `malloc` when it is the message's last extension.
- Reach: root (a PF_KEY socket).
- Verified: read.
- Port's handling: `sys/net/pfkeyv2.rs:158` copies `sa_len` bytes.
- Severity: cosmetic (an out-of-bounds read that is never used).
- Fix: copy `SA_LEN` bytes into a zeroed destination.

### EXT-20: `clock_ymdhms_to_secs` trusts the RTC's month and year

- Where: `sys/kern/clock_subr.c:104`, `clock_ymdhms_to_secs`.
- What: `for (i = 1; i < dt->dt_mon; i++) days += days_in_month(i);` (104-105) reads past
  `month_days[12]` for a month above 13. `(days * 24 + dt->dt_hour) * 60` (109-112) is `int`
  arithmetic and overflows past about year 6053.
- Reach: firmware/hardware. amd64 `rtcgettime` passes the CMOS month unchecked
  (`isa/clock.c:454-461`), and arm64's `efi_gettime` bounds the year only from below.
- Verified: read.
- Port's handling: `sys/kern/clock_subr.rs:71-73` uses i64 and ignores months past 12.
- Severity: wrong result.
- Fix: compute in `time_t`, and reject months outside 1..12 in `rtcgettime`.

### EXT-21: `userconf_number` accepts a digit equal to the base

- Where: `sys/kern/subr_userconf.c:338`, `userconf_number`.
- What: `if (cc > base) return (-1);` (338-339) should be `>=`, so "08" is octal 8. The
  overflow test (344) covers only negatives.
- Reach: root (UKC, `boot -c` or `config -e`).
- Verified: read.
- Port's handling: same fault. `sys/kern/subr_userconf.rs:85` keeps it, and a test pins it.
- Severity: wrong result.
- Fix: `if (cc >= base)`.

### EXT-22: `kern.watchdog.period * 1000` overflows int

- Where: `sys/kern/kern_watchdog.c:55` (and 103), `wdog_tickle`/`sysctl_wdog`.
- What: `timeout_add_msec(&wdog_timeout, wdog_period * 1000 / 2)` with a period the sysctl
  allows up to INT_MAX (82-83).
- Reach: root, with a driver that does not clamp the period (`ipmi_watchdog`).
- Verified: read.
- Port's handling: `sys/kern/kern_watchdog.rs:63` computes in 64 bits.
- Severity: wrong result.
- Fix: bound the sysctl to `INT_MAX / 1000`.

### EXT-23: `tcp_softtso_chop` keeps the IP header pointer across `m_pullup`

- Where: `sys/netinet/tcp_output.c:1195`, `tcp_softtso_chop`.
- What: `ip = mtod(m0, struct ip *);` (1195) is taken before `m_pullup(m0, iphlen +
  sizeof(*th))` (1230), which may move or free the data. `*mhip = *ip;` (1290) and
  `ip->ip_len = ...` (1319) use the stale pointer, and the options copy (1272) may run past
  what was pulled up.
- Reach: remote if a producer of M_TCP_TSO packets with headers split across mbufs exists;
  none was found in the tree (the C calls the pullup "paranoia").
- Verified: claimed (missing: a reachable producer).
- Port's handling: `sys/netinet/tcp_output.rs:126` copies the header out and back.
- Severity: memory corruption, if reachable.
- Fix: re-take `ip`/`ip6` after the pullup, and pull up `hlen` before copying options.

## file systems

### EXT-24: the NFS client leaks the reply of a denied RPC

- Where: `sys/nfs/nfs_socket.c:988`, `nfs_request`.
- What: on `MSG_DENIED`, `infop->nmi_mrep = NULL; goto nfsmout1;` (993-994). `nfsmout1:`
  frees only `r_mreq` (1050-1053), and the reply chain is lost.
- Reach: remote (a server, or a spoofer that matches the xid).
- Verified: read.
- Port's handling: `sys/nfs/nfs_socket.rs:89` frees it.
- Severity: leak.
- Fix: `m_freem(info.nmi_mrep)` before `goto nfsmout1`.

### EXT-25: `nfsrv_read` leaks its first reply when the read fails

- Where: `sys/nfs/nfs_serv.c:751`, `nfsrv_read`.
- What: the reply and its data clusters are built at 686. A failing `VOP_READ` or GETATTR
  (748-754) goes to `vbad`, whose `nfsm_reply` (779) overwrites `*mrq`.
- Reach: remote. A client repeats a READ of a file whose read fails (an I/O error, or a
  bad block map).
- Verified: read.
- Port's handling: `sys/nfs/nfs_serv.rs:100` frees the first reply.
- Severity: leak.
- Fix: `m_freem(*mrq)` on the `vbad` path.

### EXT-26: CREATE GUARDED on an existing name leaves the vnode locked

- Where: `sys/nfs/nfs_serv.c:1207`, `nfsrv_create`.
- What: the GUARDED case sets `error = EEXIST` (1058-1060) and skips `nfsm_srvsattr`, so
  `va.va_size != -1` (1207) is false, and `if (!error) { ... vput(vp); }` (1221) is skipped.
  The locked vnode is never released.
- Reach: remote (any v3 client on an export it can search). Later access to the file
  hangs.
- Verified: read.
- Port's handling: `sys/nfs/nfs_serv.rs:103` releases it.
- Severity: leak (a locked vnode).
- Fix: `vput(vp)` when `error` is set there.

### EXT-27: `nfsrv_create` frees the name buffer twice

- Where: `sys/nfs/nfs_serv.c:1151`, `nfsrv_create` (the v2 device/FIFO path).
- What: after a failed `VOP_MKNOD` (1145), `pool_put(&namei_pool, nd.ni_cnd.cn_pnbuf);`
  (1150-1153). `ufs_makeinode` has already freed it on every error path
  (`ufs_vnops.c:1760, 1771, 1808`); `nfsrv_mknod` (1389-1393) rightly does not free it.
- Reach: remote. A v2 CREATE with a FIFO mode needs no root (1138), and `VOP_MKNOD` fails
  when the file system is out of inodes or over quota, or on an I/O error.
- Verified: read.
- Port's handling: `sys/nfs/nfs_serv.rs:111` does not free it again.
- Severity: memory corruption (a double free into `namei_pool`).
- Fix: drop the `pool_put` at 1150-1153.

### EXT-28: `nfsrv_rename`'s error path leaks a directory reference and the name

- Where: `sys/nfs/nfs_serv.c:1740`, `nfsrv_rename`.
- What: `nfsmout:` releases `ni_dvp` only `if (fromnd.ni_dvp != fdirp)` (1740); for v3 they
  are always equal, so one reference leaks. The SAVESTART name buffer also leaks.
- Reach: remote (a RENAME whose second file handle is malformed, 1618).
- Verified: read.
- Port's handling: `sys/nfs/nfs_serv.rs:106` releases both.
- Severity: leak (the export can no longer be unmounted).
- Fix: always `vrele(fromnd.ni_dvp)` and free the pnbuf there.

### EXT-29: `nfs_lookitup` calls `vput(NULL)`

- Where: `sys/nfs/nfs_vnops.c:2942`, `nfs_lookitup`.
- What: a failed v3 LOOKUP skips the `if (npp && !error)` block (2898), and `nfsmout:` does
  `if (error) { if (newvp == dvp) vrele(newvp); else vput(newvp); }` (2937-2942) with
  `newvp` NULL.
- Reach: remote. `nfs_create`/`nfs_mknodrpc` call it when the reply has no file handle
  (RFC 1813 allows that), and so does `nfs_mkdir`. The crash follows when that LOOKUP
  fails: a server error, a soft-mount timeout, or a malicious server.
- Verified: read.
- Port's handling: `sys/nfs/nfs_vnops.rs:106` skips it.
- Severity: crash.
- Fix: `if (error && newvp != NULL)`.

### EXT-30: `nfsrv_zapsock` leaks a partial TCP record

- Where: `sys/nfs/nfs_syscalls.c:517`, `nfsrv_zapsock`.
- What: it frees `ns_nam`, `ns_raw` and `ns_rec` (517-525) but not `ns_frag`, the fragments
  of a record still waiting for its last fragment (`nfs_socket.c:1786-1797`).
- Reach: remote. Any host that can reach nfsd's TCP port sends a record mark without the
  last-fragment bit plus some data, then closes the connection.
- Verified: read.
- Port's handling: same fault (`sys/nfs/nfs_syscalls.rs:767-792`).
- Severity: leak.
- Fix: `m_freem(slp->ns_frag); slp->ns_frag = NULL;`.

### EXT-31: `krpc_call` leaks the reply on its EBADRPC paths

- Where: `sys/nfs/krpc_subr.c:432`, `krpc_call`.
- What: after `gotreply:`, three checks do `error = EBADRPC; goto out;` (431-432, 437-438,
  445-446), and `out:` does not free the reply `m`.
- Reach: remote (a server or spoofer during a diskless boot).
- Verified: read.
- Port's handling: `sys/nfs/krpc_subr.rs:85` frees it.
- Severity: leak.
- Fix: `m_freem(m)` on those paths.

### EXT-32: `ext4_bmapext` uses an uninitialised extent path

- Where: `sys/ufs/ext2fs/ext2fs_bmap.c:110`, `ext4_bmapext`.
- What: `struct ext4_extent_path path;` (99) is not initialised. `ext4_ext_find_extent`
  returns early on a bad magic (`ext2fs_extents.c:141`), and `if ((ep = path.ep_ext) ==
  NULL)` (111) reads stack garbage and dereferences it.
- Reach: device (a crafted ext2/ext4 image; any lookup in a directory with a bad extent
  header).
- Verified: read.
- Port's handling: `sys/ufs/ext2fs/ext2fs_bmap.rs:76` starts from an empty path.
- Severity: crash.
- Fix: `memset(&path, 0, sizeof(path))`.

### EXT-33: ext4 extent lookups leave the leaf buffer busy

- Where: `sys/ufs/ext2fs/ext2fs_bmap.c:111`, `ext4_bmapext`; also `ext2fs_subr.c:79` and
  `ext2fs_readwrite.c:192`.
- What: for trees of depth 1 or more, `path->ep_bp` comes back held. `ext4_bmapext` never
  releases it, and the other two callers release it only on some paths.
- Reach: device (a crafted image with an empty leaf; `ext4_bmapext` leaks on any deep file).
  The next read of the block, and the unmount, sleep for ever.
- Verified: read.
- Port's handling: `sys/ufs/ext2fs/ext2fs_bmap.rs:76` and `ext2fs_readwrite.rs:67` release it
  on every path.
- Severity: leak (a busy buffer, which hangs).
- Fix: `brelse(path.ep_bp)` before every return.

### EXT-34: the ext4 extent binary search trusts `eh_ecount`

- Where: `sys/ufs/ext2fs/ext2fs_extents.c:73`, `ext4_ext_binsearch` (and 51).
- What: `r = (struct ext4_extent *)(char *)(ehp + 1) + ehp->eh_ecount - 1;` (73) with
  `eh_ecount` up to 65535 and never checked against the node; `ei_leaf` from past the node
  picks the next block to read.
- Reach: device (a crafted ext4 image).
- Verified: read.
- Port's handling: `sys/ufs/ext2fs/ext2fs_extents.rs:103` checks the count.
- Severity: crash (out-of-bounds read).
- Fix: reject `eh_ecount > eh_max` or more entries than the node holds.

### EXT-35: `ext2fs_readdir` reads past its directory buffer

- Where: `sys/ufs/ext2fs/ext2fs_lookup.c:175`, `ext2fs_readdir`.
- What: the loop checks only the entry's start (169), then `ext2fs_dirconv2ffs(dp, &dstd);`
  (175) copies the header and up to 255 name bytes past `readcnt` and past `dirbuf` (162),
  into the `d_name` returned to the user.
- Reach: device (a crafted image; an entry starting near the end of the read).
- Verified: read.
- Port's handling: `sys/ufs/ext2fs/ext2fs_lookup.rs:93` stops with EIO.
- Severity: crash (out-of-bounds read; the bytes are disclosed to the user).
- Fix: require `reclen >= EXT2FS_DIRSIZ(namlen)` and the entry within `readcnt`.

### EXT-36: `ext2fs_search_dirblock` loops on a zero reclen

- Where: `sys/ufs/ext2fs/ext2fs_lookup.c:638`, `ext2fs_search_dirblock`.
- What: a mangled entry advances `offset` and does `continue;` (631-638), which skips the
  recompute of `ep` at the end of the loop (696), so the same entry is examined again.
  `ufs_dirbad` panics only on a read-write mount.
- Reach: device (a crafted image mounted read-only; any lookup in that directory spins
  holding the directory lock).
- Verified: read.
- Port's handling: `sys/ufs/ext2fs/ext2fs_lookup.rs:96` takes the next entry from `offset`.
- Severity: crash (a kernel hang).
- Fix: recompute `ep` from `offset` in the loop.

### EXT-37: `ext2fs_reload` keeps the superblock buffer busy

- Where: `sys/ufs/ext2fs/ext2fs_vfsops.c:464`, `ext2fs_reload`.
- What: the superblock is `bread` (447) and never released, on success or failure, and
  `e2fs_sbfill` replaces `e2fs_gd` without freeing the old array.
- Reach: root (`mount -u -o reload` of a read-only ext2fs). A second reload or the unmount
  sleeps for ever.
- Verified: read.
- Port's handling: `sys/ufs/ext2fs/ext2fs_vfsops.rs:77` releases both.
- Severity: leak (a busy buffer, which hangs).
- Fix: `brelse` after `e2fs_sbload`, and free the old `e2fs_gd`.

### EXT-38: `fusefs_close` from the descriptor garbage collector dereferences a NULL proc

- Where: `sys/miscfs/fuse/fuse_vnops.c:333`, `fusefs_close`; the dereference is in
  `fb_setup` (`fusebuf.c:123-125`).
- What: `fb_setup(0, ip->i_number, FUSE_FLUSH, ap->a_p)` then `p->p_tid` and
  `p->p_ucred`. `a_p` is NULL when `unp_gc` closes the file (`uipc_usrreq.c:1381
  closef(fp, NULL)` reaches `VOP_CLOSE(..., NULL)`, `vfs_vnops.c:298`).
- Reach: user. Open a file for writing on a FUSE mount, send the descriptor over an AF_UNIX
  socket, close both without receiving it.
- Verified: read.
- Port's handling: `sys/miscfs/fuse/fuse_vnops.rs:219` uses `curproc`.
- Severity: crash.
- Fix: use `curproc` when `ap->a_p` is NULL.

### EXT-39: `fusefs_readdir` leaks the fusebuf when the buffer fills

- Where: `sys/miscfs/fuse/fuse_vnops.c:881`, `fusefs_readdir`.
- What: `if (uio->uio_resid < de.d_reclen) goto out;` (880-881) skips `fb_delete(fbuf);`
  (906).
- Reach: device (the normal case for a large FUSE directory; each getdents leaks a fusebuf).
- Verified: read.
- Port's handling: `sys/miscfs/fuse/fuse_vnops.rs:61` frees it.
- Severity: leak.
- Fix: `fb_delete(fbuf)` before `goto out`.

### EXT-40: `fusefs_mkdir` leaks the fusebuf on a bad node id

- Where: `sys/miscfs/fuse/fuse_vnops.c:1532`, `fusefs_mkdir`.
- What: a reply with node id 0 or the root goes to `out:` (1532-1533) without
  `fb_delete(fbuf)`; symlink frees it (753-757).
- Reach: device (the FUSE daemon's reply).
- Verified: read.
- Port's handling: `sys/miscfs/fuse/fuse_vnops.rs:61` frees it.
- Severity: leak.
- Fix: `fb_delete(fbuf)` there.

### EXT-41: `fuse_device_cleanup` corrupts the queues

- Where: `sys/miscfs/fuse/fuse_device.c:136` (and 117), `fuse_device_cleanup`.
- What: the loop removes `f` and keeps it as `lprev`, then calls
  `SIMPLEQ_REMOVE_AFTER(&fd->fd_fbufs_wait, lprev, fb_next)` (133-137) on an element already
  off the list. From the second element on nothing is unlinked, `sqh_last` can point into a
  removed element, and the woken waiters free fbufs that are still linked. `fusewrite` later
  walks them (370) and dereferences `fd->fd_fmp` NULL (404).
- Reach: root (`umount -f` of a FUSE mount with two or more requests waiting, then a daemon
  write).
- Verified: read.
- Port's handling: `sys/miscfs/fuse/fuse_device.rs:55` removes the head until empty; `:61`
  answers ENODEV with no mount.
- Severity: memory corruption (use after free).
- Fix: `while ((f = SIMPLEQ_FIRST(q)) != NULL) SIMPLEQ_REMOVE_HEAD(q, fb_next);`, and fail
  `fusewrite` when `fd_fmp` is NULL.

### EXT-42: cleared FORGET and INIT fusebufs are never freed

- Where: `sys/miscfs/fuse/fuse_device.c:120`, `fuse_device_cleanup`.
- What: cleanup sets `fb_err = ENXIO` and wakes the fusebufs, but FORGET and INIT have no
  sleeper and are freed only in `fuseread` (323) and `fusewrite` (482).
- Reach: device (the daemon closes /dev/fuse before reading them).
- Verified: read.
- Port's handling: same fault, kept as in C (`sys/miscfs/fuse/fuse_device.rs:59-60`).
- Severity: leak.
- Fix: `fb_delete` those types in cleanup.

### EXT-43: `fusefs_lookup` writes an uninitialised vtype on a RENAME to `..`

- Where: `sys/miscfs/fuse/fuse_lookup.c:161`, `fusefs_lookup`.
- What: `enum vtype nvtype;` (48) is set only on the daemon path (123). For `..` the code
  takes `i_parent_cache` (77), and under RENAME does `tdp->v_type = nvtype;` (161).
- Reach: user (`rename(x, "dir/..")` with write access on a FUSE directory).
- Verified: read.
- Port's handling: `sys/miscfs/fuse/fuse_lookup.rs:43` leaves `v_type` alone.
- Severity: wrong result (a corrupted directory vnode type).
- Fix: set `v_type` only from a daemon reply.

### EXT-44: a `..` that is not a directory leaks the locked vnode

- Where: `sys/miscfs/fuse/fuse_lookup.c:185`, `fusefs_lookup`.
- What: `VFS_VGET` (173) and then `error = EIO` for a non-directory (175), followed by
  `goto reclaim;` (185), which never does `vput(tdp)`.
- Reach: device (the daemon reports the parent as a non-directory).
- Verified: read.
- Port's handling: same fault, kept as in C (`sys/miscfs/fuse/fuse_lookup.rs:46-47`).
- Severity: leak (a locked vnode).
- Fix: `vput(tdp)` before `goto reclaim`.

### EXT-45: an LZNT1 back reference reads before the output buffer

- Where: `sys/ntfs/ntfs_compr.c:78`, `ntfs_uncompblock`.
- What: `boff = -1 - (GET_UINT16(cbuf + cpos) >> dshift);` (74) and then `buf[pos] =
  buf[pos + boff];` (78), with no check that `pos + boff >= 0`. The stored-block `memcpy`
  (59) and `cpos < len + 3` also read past the input.
- Reach: device (reading a compressed file from a crafted NTFS image). The bytes read land in
  the user's data.
- Verified: read.
- Port's handling: `sys/ntfs/ntfs_compr.rs:91` reads 0 there.
- Severity: crash (out-of-bounds read, disclosed to the user).
- Fix: reject `pos + boff < 0`, and bound reads by the compressed size.

### EXT-46: `ntfs_uncompunit` writes 4 KB blocks into a smaller unit buffer

- Where: `sys/ntfs/ntfs_compr.c:98`, `ntfs_uncompunit`.
- What: the loop (98-104) decompresses whole 4096-byte blocks into `uup`, which is
  `malloc(ntfs_cntob(NTFS_COMPUNIT_CL))` (`ntfs_subr.c:1552`). That is not a multiple of
  4096 for unusual geometry (e.g. bps 384), which mount never validates.
- Reach: device (a crafted boot sector plus a compressed file; the geometry is claimed, not
  tested).
- Verified: read.
- Port's handling: panics instead: `sys/ntfs/ntfs_compr.rs:170-174` slices past the unit;
  not in its Deviations.
- Severity: memory corruption.
- Fix: validate bps and spc at mount, or bound the last block by the unit.

### EXT-47: the MFT attribute walk has no bound or progress check

- Where: `sys/ntfs/ntfs_subr.c:336`, `ntfs_loadntnode`.
- What: `off += ap->a_hdr.reclen;` (336) is never checked against the record, and reclen 0
  never advances; each pass allocates an ntvattr from offsets it also trusts.
- Reach: device (a crafted NTFS image).
- Verified: read.
- Port's handling: `sys/ntfs/ntfs_subr.rs:117` answers EINVAL.
- Severity: crash (a hang with unbounded allocation, or an out-of-bounds read).
- Fix: require `reclen >= sizeof(a_hdr)` and `off + reclen` within the record.

### EXT-48: the attribute-list walk loops or dereferences NULL

- Where: `sys/ntfs/ntfs_subr.c:207`, `ntfs_ntvattrget`.
- What: `len -= aalp->reclen;` (207). Reclen 0 never advances, and reclen > len wraps the
  `size_t` and leaves `nextaalp` NULL (205) for the next pass.
- Reach: device (a crafted `$ATTRIBUTE_LIST`).
- Verified: read.
- Port's handling: `sys/ntfs/ntfs_subr.rs:115-116` ends the scan.
- Severity: crash.
- Fix: stop when `reclen == 0 || reclen > len`.

### EXT-49: the index root buffer is sized by `ir_size` but filled by `va_datalen`

- Where: `sys/ntfs/ntfs_subr.c:876`, `ntfs_ntlookupfile`.
- What: `rdbuf = malloc(blsize, ...)` with `blsize = ir_size` (873-876), then
  `ntfs_readattr(..., rdsize = va_datalen, rdbuf, ...)` (879-883). `ntfs_ntreaddir`
  allocates the larger of the two (1158).
- Reach: device (a crafted directory; any lookup overflows the heap by up to an MFT record).
- Verified: read.
- Port's handling: `sys/ntfs/ntfs_subr.rs:97-98` bounds copies by the buffer.
- Severity: memory corruption.
- Fix: allocate `MAX(va_datalen, ir_size)`.

### EXT-50: the NTFS index-entry walks loop on a zero-length entry

- Where: `sys/ntfs/ntfs_subr.c:893` (and 1229), `ntfs_ntlookupfile`, `ntfs_ntreaddir`.
- What: `aoff += iep->reclen` (893) does not progress when `reclen` is 0, and the
  `continue`s keep the loop going.
- Reach: device (a crafted directory entry).
- Verified: read.
- Port's handling: `sys/ntfs/ntfs_subr.rs:115` ends the scan.
- Severity: crash (a kernel hang).
- Fix: stop on a reclen below the header or past `rdsize`.

### EXT-51: a failed `uiomove` leaves an NTFS buffer busy

- Where: `sys/ntfs/ntfs_subr.c:1415`, `ntfs_readntvattr_plain`.
- What: `if (error != 0) break;` (1413-1415) leaves the loop before `brelse(bp)`.
- Reach: user. Any user who can read a file on an NTFS mount does `read(fd, bad_address,
  n)`; the next read of that cluster, and the unmount, hang.
- Verified: read.
- Port's handling: `sys/ntfs/ntfs_subr.rs:125` releases it.
- Severity: leak (a busy buffer, which hangs).
- Fix: `brelse(bp)` before the `break`.

### EXT-52: `ntfs_mountfs` divides by a zero sector size

- Where: `sys/ntfs/ntfs_vfsops.c:312`, `ntfs_mountfs`.
- What: `ntm_bpmftrec = (1 << (-cpr)) / ntmp->ntm_bps;` (312), where `ntm_bps` comes from
  the boot sector and is never checked.
- Reach: device (a crafted image with `bf_bps` 0).
- Verified: read.
- Port's handling: `sys/ntfs/ntfs_vfsops.rs:96` answers EINVAL.
- Severity: crash.
- Fix: reject a zero or non-power-of-two bps and spc.

### EXT-53: an `$AttrDef` name is copied without a bound

- Where: `sys/ntfs/ntfs_vfsops.c:408`, `ntfs_mountfs`.
- What: `do { ntmp->ntm_ad[i].ad_name[j] = ad.ad_name[j]; } while(ad.ad_name[j++]);`
  (407-409) into a 64-byte name, until a NUL the disk may not have.
- Reach: device (a crafted `$AttrDef` with a 64-character name).
- Verified: read.
- Port's handling: `sys/ntfs/ntfs_vfsops.rs:99-100` stops at the maximum length.
- Severity: memory corruption.
- Fix: bound `j` by the array and NUL-terminate.

### EXT-54: the `$AttrDef` vnode stays locked on the error path

- Where: `sys/ntfs/ntfs_vfsops.c:388` (and 405), `ntfs_mountfs`.
- What: `VFS_VGET` (378), then `goto out1;` (388, 405) without `vput(vp)`. The mount is
  freed while the vnode survives.
- Reach: device (an `$AttrDef` without its terminating entry).
- Verified: read.
- Port's handling: `sys/ntfs/ntfs_vfsops.rs:97-99` releases it.
- Severity: leak (a locked vnode pointing at freed mount data).
- Fix: `vput(vp)` before `goto out1`.

### EXT-55: `ntfs_vgetex` releases the ntnode twice

- Where: `sys/ntfs/ntfs_vfsops.c:759`, `ntfs_vgetex`.
- What: `ntfs_ntput(ip);` (742), then on a failed `getnewvnode` (756) `ntfs_frele(fp);
  ntfs_ntput(ip);` (758-759) again, unlocking a lock it does not hold.
- Reach: alloc-failure (`getnewvnode` returns ENFILE under vnode pressure).
- Verified: read.
- Port's handling: same fault, kept "as the C does" (`sys/ntfs/ntfs_vfsops.rs:103`).
- Severity: crash.
- Fix: drop the second `ntfs_ntput`.

### EXT-56: `iso_mountfs` leaks the primary descriptor, and the mount hangs

- Where: `sys/isofs/cd9660/cd9660_vfsops.c:334`, `iso_mountfs`.
- What: `pribp` (282) is released only at 355. A bad block size (333-334) or an earlier
  `goto out` skips it, and `VOP_CLOSE` (432) then sleeps on the busy buffer in
  `vinvalbuf`.
- Reach: device (a crafted ISO with a bad logical block size). mount(2) hangs
  uninterruptibly.
- Verified: read.
- Port's handling: `sys/isofs/cd9660/cd9660_vfsops.rs:74` releases it.
- Severity: leak (a busy buffer, which hangs).
- Fix: `if (pribp) brelse(pribp);` at `out:`.

### EXT-57: `udf_readatoffset` hands out bytes past the buffer for an unaligned offset

- Where: `sys/isofs/udf/udf_vnops.c:1271`, `udf_readatoffset`.
- What: it reads `*size` bytes from the block holding `offset` (1260-1264), then
  `*data = &bp1->b_data[offset % ump->um_bsize];` (1271) without taking `offset % bsize` off
  `*size`. `udf_read` copies `size` bytes out (475).
- Reach: user. Any user reads a regular UDF file at an unaligned offset (lseek 1000, read
  2048). Well-formed media is enough.
- Verified: read.
- Port's handling: `sys/isofs/udf/udf_vnops.rs:60` clamps the size.
- Severity: crash (out-of-bounds read; kernel memory disclosed to the user).
- Fix: `*size = min(*size, bp->b_bcount - offset % bsize)`.

### EXT-58: embedded UDF file data ignores the offset

- Where: `sys/isofs/udf/udf_vnops.c:1252`, `udf_readatoffset`.
- What: for data embedded in the file entry it returns the start, `*size = l_ad` (1252),
  whatever `offset` is.
- Reach: user (well-formed media; a read not at 0 of a small file, a directory, a VAT).
- Verified: read.
- Port's handling: `sys/isofs/udf/udf_vnops.rs:64` honours the offset.
- Severity: wrong result.
- Fix: add `offset` to `*data`, take it off `*size`.

### EXT-59: `udf_read` spins on an embedded file with no data

- Where: `sys/isofs/udf/udf_vnops.c:468`, `udf_read`.
- What: `while (uio->uio_offset < fsize && uio->uio_resid > 0)` (468) with
  `udf_readatoffset` returning size 0; `uiomove(data, 0, uio)` makes no progress.
- Reach: device (a crafted image: `inf_len > 0`, `l_ad` 0).
- Verified: read.
- Port's handling: `sys/isofs/udf/udf_vnops.rs:82` stops.
- Severity: crash (a kernel hang).
- Fix: stop on a zero-length chunk.

### EXT-60: `udf_getfid` copies a negative fragment size

- Where: `sys/isofs/udf/udf_vnops.c:659`, `udf_getfid`.
- What: `frag_size = ds->size - ds->off;` (647) is negative when `ds->off` was rounded past
  the end; the check `>= um_bsize` (648) passes it, and `bcopy(fid, ds->buf, frag_size)`
  (659) copies a huge size. The fragment buffer also leaks on two error returns.
- Reach: device (a crafted directory extent).
- Verified: read.
- Port's handling: `sys/isofs/udf/udf_vnops.rs:78-80` rejects it and frees the buffer.
- Severity: crash.
- Fix: reject `ds->off > ds->size`.

### EXT-61: `udf_readdir` returns ERESTART for ever

- Where: `sys/isofs/udf/udf_vnops.c:816`, `udf_readdir`.
- What: when "." does not fit, `if (error) break;` (815-816) leaves the loop with `-1`
  before the `if (error == -1) error = 0;` below. `-1` is ERESTART, so getdents restarts for
  ever.
- Reach: user (getdents at offset 0 with a buffer smaller than ".").
- Verified: read.
- Port's handling: same fault, kept (`sys/isofs/udf/udf_vnops.rs:74-76`).
- Severity: wrong result (an endless restart).
- Fix: map -1 to 0 on that path too.

### EXT-62: a month past 12 reads past `mon_lens`

- Where: `sys/isofs/udf/udf_vnops.c:301`, `udf_timetotimespec`.
- What: `for (i = 1; i < time->month; i++) t->tv_sec += mon_lens[lpyear][i] * ...`
  (300-301) with an on-disk month up to 255.
- Reach: device (stat on a crafted image).
- Verified: read.
- Port's handling: `sys/isofs/udf/udf_vnops.rs:85` ignores such months.
- Severity: wrong result.
- Fix: clamp the month to 1..12.

### EXT-63: `udf_bmap_internal` reads one descriptor past `l_ad`

- Where: `sys/isofs/udf/udf_vnops.c:1328` (and 1358), `udf_bmap_internal`.
- What: `if (ad_offset > l_ad)` (1328) lets `ad_offset == l_ad` through, and the descriptor
  read there lies past the copy.
- Reach: device.
- Verified: read.
- Port's handling: `sys/isofs/udf/udf_vnops.rs:69-70` reads zeros.
- Severity: wrong result (a small out-of-bounds read).
- Fix: `ad_offset + sizeof(ad) > l_ad`.

### EXT-64: `udf_vget` trusts `l_ea + l_ad`

- Where: `sys/isofs/udf/udf_vfsops.c:618`, `udf_vget`.
- What: `size = l_ea + l_ad` (595) from disk; then `malloc(size + UDF_EXTFENTRY_SIZE, ...,
  M_NOWAIT | M_ZERO)` (609) panics for a large size, and `bcopy(bp->b_data, up->u_fentry,
  size + UDF_EXTFENTRY_SIZE)` (618) reads past the one-block buffer for a moderate one.
- Reach: device (any lookup on a crafted image).
- Verified: read.
- Port's handling: `sys/isofs/udf/udf_vfsops.rs:82` copies at most the buffer.
- Severity: crash.
- Fix: reject sizes over the block before allocating.

### EXT-65: `udf_vat_read` calls `brelse(NULL)`

- Where: `sys/isofs/udf/udf_subr.c:264`, `udf_vat_read`.
- What: `udf_readatoffset` returns `bp == NULL` for embedded data (245), and the success
  path does `brelse(bp);` (264); `brelse` dereferences it (`vfs_bio.c:733`).
- Reach: device (mounting an image whose VAT is embedded).
- Verified: read.
- Port's handling: `sys/isofs/udf/udf_subr.rs:64-65` releases only a buffer.
- Severity: crash.
- Fix: `if (bp != NULL) brelse(bp);`.

### EXT-66: the VAT copy keeps a file entry the vnode cache frees

- Where: `sys/isofs/udf/udf_subr.c:204`, `udf_vat_get`.
- What: `*ump->um_vat = *up;` (204) copies the unode with its `u_fentry` pointer. When the
  VAT vnode is recycled, `udf_reclaim` frees `u_fentry` (`udf_vnops.c:1205-1206`), and
  every later VAT lookup reads freed memory.
- Reach: user (a packet-written disc with a VAT under vnode pressure; normal media).
- Verified: read.
- Port's handling: `sys/isofs/udf/udf_subr.rs:60-61` keeps its own copy.
- Severity: memory corruption (use after free).
- Fix: copy `u_fentry`, or hold a reference on the VAT vnode.

### EXT-67: UDF partition maps and the sparing table are read past their buffers

- Where: `sys/isofs/udf/udf_vfsops.c:794` (and 762, 821), `udf_get_spartmap`,
  `udf_find_partmaps`.
- What: `malloc(st_size, ..., M_NOWAIT)` (762) panics for a large size. `for (i = 0; i <
  rt_l; i++)` (794) reads up to 65535 entries past the table, and the map walk (821) runs
  past the 2048-byte LVD.
- Reach: device (mounting a crafted image).
- Verified: read.
- Port's handling: `sys/isofs/udf/udf_vfsops.rs:76-79` bounds them.
- Severity: crash.
- Fix: bound `rt_l` by `st_size`, the walk by the map table, and use `M_CANFAIL`.

### EXT-68: `udf_mountfs`'s error path leaks the tables

- Where: `sys/isofs/udf/udf_vfsops.c:433`, `udf_mountfs`.
- What: `bail:` frees `ump` (433) but not `um_stbl` and `um_vat`, which `udf_unmount` frees.
- Reach: device (a crafted image whose mount fails late).
- Verified: read.
- Port's handling: `sys/isofs/udf/udf_vfsops.rs:85` frees them.
- Severity: leak.
- Fix: free both at `bail:`.

### EXT-69: UDF's `vfc_datasize` is `struct iso_args`'s

- Where: `sys/kern/vfs_init.c:91`, `vfsconflist[]`.
- What: the UDF entry gives `sizeof(struct iso_args)` (90-91), but mount_udf passes a
  16-byte `struct udf_args`; `sys_mount` copies in `vfc_datasize` bytes.
- Reach: root (args at the end of a mapped page fail with EFAULT).
- Verified: read.
- Port's handling: `sys/isofs/udf/udf_vfsops.rs:72-74` uses the right size.
- Severity: cosmetic.
- Fix: `sizeof(struct udf_args)`.

### EXT-70: `fillinusemap` calls `brelse(NULL)`

- Where: `sys/msdosfs/msdosfs_fat.c:917`, `fillinusemap`.
- What: with `pm_maxcluster` 1 the loop (892) never reads a block, and `brelse(bp);` (917)
  gets NULL.
- Reach: device (a crafted FAT image with a data area smaller than one cluster).
- Verified: read.
- Port's handling: `sys/msdosfs/msdosfs_fat.rs:111` skips it.
- Severity: crash.
- Fix: `if (bp) brelse(bp);`, or reject zero clusters at mount.

## USB, storage and network devices

### EXT-71: the READ TOC length from the drive overruns the TOC

- Where: `sys/scsi/cd.c:1581`, `cd_load_toc`.
- What: `n = ending_track - starting_track + 2; len = n * sizeof(struct cd_toc_entry) +
  sizeof(toc->header);` (1580-1581) from drive-supplied bytes, up to 2060 bytes. That is
  more than the 804-byte `struct cd_toc` in a 1024-byte DMA item; `cd_read_toc` zeroes and
  DMAs `len` bytes into it (1552).
- Reach: device (a drive or USB stick reporting more than 127 tracks).
- Verified: read.
- Port's handling: `sys/scsi/cd.rs:126` clamps to the TOC.
- Severity: memory corruption.
- Fix: clamp `len` to `sizeof(*toc)`.

### EXT-72: CDIOREADTOCENTRYS converts entries past the TOC

- Where: `sys/scsi/cd.c:913`, `cdioctl`.
- What: `for (ntracks = ending_track - starting_track + 1; ntracks >= 0; ntracks--) { cte =
  &toc->entries[ntracks]; ... cte->addr.lba = betoh32(cte->addr.lba); }` (910-922) writes up
  to `entries[256]` of a 100-entry array.
- Reach: device (a drive reporting track numbers over 99, read by a user with access to
  the cd device).
- Verified: read.
- Port's handling: `sys/scsi/cd.rs:128` skips such indexes.
- Severity: memory corruption.
- Fix: bound `ntracks` by MAXTRACK and the length read.

### EXT-73: `cd_play_tracks` indexes the TOC with a user's track number

- Where: `sys/scsi/cd.c:1409`, `cd_play_tracks`.
- What: `strack` from the ioctl is only checked against `etrack`, then
  `toc->entries[strack].addr.msf` (1409) is read up to index 255 of 100.
- Reach: user (CDIOCPLAYTRACKS with start and end track 200, on a normal disc).
- Verified: read.
- Port's handling: `sys/scsi/cd.rs:129` answers EINVAL.
- Severity: wrong result (an out-of-bounds read sent to the drive).
- Fix: reject `strack > ending_track`.

### EXT-74: `dvd_read_bca` leaks its DMA buffer

- Where: `sys/scsi/cd.c:1990`, `dvd_read_bca`.
- What: a BCA length outside 12..188 does `return EIO;` (1989-1990), skipping `dma_free`.
- Reach: device.
- Verified: read.
- Port's handling: `sys/scsi/cd.rs:132` frees it.
- Severity: leak.
- Fix: `error = EIO; goto done;`.

### EXT-75: MODE SENSE(10) and MODE SELECT(10) trust the device's lengths

- Where: `sys/scsi/scsi_base.c:1224` (and 1383), `scsi_mode_sense_big_page`,
  `scsi_mode_select_big`.
- What: `header_length = sizeof(*hdr) + _2btol(hdr->blk_desc_len); page = (u_int8_t *)hdr +
  header_length;` (1223-1224) and `if ((*page & SMS_PAGE_CODE) != pg_code)` (1229) read up
  to 64 KB past a 254-byte buffer, and `len = _2btol(data->data_length) + 2;` (1383) makes
  MODE SELECT DMA that much out of it.
- Reach: device (a malicious SCSI, ATAPI or umass device, at attach or on ioctls).
- Verified: read.
- Port's handling: `sys/scsi/scsi_base.rs:88` and `:94` clamp.
- Severity: crash (an out-of-bounds read; kernel memory sent to the device).
- Fix: clamp both lengths to the buffer.

### EXT-76: `viscpy` reads past the INQUIRY field

- Where: `sys/scsi/sd.c:1374`, `viscpy`.
- What: unprintable bytes advance `src` without decrementing `len` (1374), so an
  all-unprintable field runs on into the next one.
- Reach: device.
- Verified: read.
- Port's handling: `sys/scsi/sd.rs:103` stops at the field's end.
- Severity: cosmetic.
- Fix: count every source byte.

### EXT-77: a deferred RAID 5/6 write is queued twice after a read error

- Where: `sys/dev/softraid_raid5.c:486` (and `softraid_raid6.c:622`), `sr_raid5_rw`.
- What: the write `W` goes on `sd_wu_defq` (484-486). When its read fails,
  `sr_raid_recreate_wu(wu->swu_collider)` (`softraid.c:2270`) runs `sr_raid5_rw(W)` again,
  which inserts `W` a second time while it is still queued; then `sr_raid_startwu` (2273)
  starts it.
- Reach: firmware/hardware (a disk read error during a RAID 5/6 write, which RAID exists to
  survive).
- Verified: read.
- Port's handling: `sys/dev/softraid_raid5.rs:68` and `softraid_raid6.rs:75` queue it once.
- Severity: memory corruption (a corrupted list, parity from incomplete reads).
- Fix: take `W` off `sd_wu_defq` before reissuing it.

### EXT-78: RAID 5 leaks a strip block on two error paths

- Where: `sys/dev/softraid_raid5.c:608` (and 824), `sr_raid5_write`, `sr_raid5_rebuild`.
- What: `xorbuf = sr_block_get(sd, len);` (608, 821) is lost by the `goto bad` paths before
  it is handed to a ccb.
- Reach: alloc-failure (and a chunk failing during a rebuild).
- Verified: read.
- Port's handling: `sys/dev/softraid_raid5.rs:64` puts it back.
- Severity: leak.
- Fix: `sr_block_put(xorbuf)` on those paths.

### EXT-79: RAID 6 leaks blocks and ccbs on allocation failure

- Where: `sys/dev/softraid_raid6.c:632` (and 740), `sr_raid6_rw`, `sr_raid6_addio`.
- What: the C's own comments: `/* XXX - can leak pbuf/qbuf on error. */` (632), `/* XXX -
  can leak data and ccb on failure. */` (740).
- Reach: alloc-failure.
- Verified: read.
- Port's handling: `sys/dev/softraid_raid6.rs:69-71` closes them.
- Severity: leak.
- Fix: free pbuf, qbuf, the ccb and its block on those paths.

### EXT-80: `sr_raid6_intr` leaks a failed read's parity opaque

- Where: `sys/dev/softraid_raid6.c:655`, `sr_raid6_intr`.
- What: `pq` is freed only `if (ccb->ccb_state == SR_CCB_OK && pq)` (655-664), and
  `sr_ccb_put` does not free `ccb_opaque`.
- Reach: firmware/hardware (any read error on a RAID 6 chunk).
- Verified: read.
- Port's handling: `sys/dev/softraid_raid6.rs:72-73` frees it.
- Severity: leak.
- Fix: free `pq` whatever the state.

### EXT-81: softraid crypto copies a KDF hint past `kdfinfo`

- Where: `sys/dev/softraid_crypto.c:368` (and 598), `sr_crypto_get_kdf`.
- What: `kdfinfo` is `malloc(bc_opaque_size)` (180 bytes; 356); the check `sizeof(scm_kdfhint)
  < genkdf.len` (365) admits 256, and `memcpy(..., &kdfinfo->genkdf, genkdf.len)` (368)
  reads up to 116 bytes past it into the on-disk metadata.
- Reach: root (bioctl with a crafted `genkdf.len`).
- Verified: read.
- Port's handling: `sys/dev/softraid_crypto.rs:79-81` copies only the union.
- Severity: wrong result (kernel heap written to disk).
- Fix: bound `genkdf.len` by the hint's size in `kdfinfo`.

### EXT-82: the key-disk metadata holding the mask key leaks unzeroed

- Where: `sys/dev/softraid_crypto.c:784`, `sr_crypto_create_key_disk`.
- What: `omi->omi_som` (759) gets the mask key (766-767), and `free(omi, ...)` (784) frees
  only the item.
- Reach: root (each `bioctl -c C -k keydisk`).
- Verified: read.
- Port's handling: `sys/dev/softraid_crypto.rs:86-87` zeroes and frees it.
- Severity: leak (key material).
- Fix: `explicit_bzero` and free `omi->omi_som` first.

### EXT-83: the old-format mask key is read at the wrong offset

- Where: `sys/dev/softraid_crypto.c:897`, `sr_crypto_read_key_disk`.
- What: `memcpy(mdd_crypto->scr_maskkey, omh + sizeof(struct sr_meta_opt_hdr), ...)`
  (896-897), where `omh` is a `struct sr_meta_opt_hdr *`, is 576 bytes in, not 24, and past
  a short item.
- Reach: device (assembling with an old-format key disk, or a crafted one).
- Verified: read (the original format's layout not checked).
- Port's handling: same fault, kept for compatibility (`sys/dev/softraid_crypto.rs:89-92`).
- Severity: wrong result.
- Fix: `(u_int8_t *)omh + sizeof(...)`, bounded by `som_length`.

### EXT-84: the block after a concat volume's end reaches `sv_chunks[no_chunk]`

- Where: `sys/dev/softraid_concat.c:140`; the cause is `sr_validate_io` (`softraid.c:4626`).
- What: `if (wu->swu_blk_end > ssd_size)` (4626) admits `blk_end == ssd_size`. The chunk loop
  then ends with `chunk == no_chunk`, `lbaoffs > chunkend` (134) is false, and `scp =
  sd->sd_vol.sv_chunks[chunk]; if (scp->src_meta...` (140-141) reads past the array.
- Reach: root (a raw SCSI READ/WRITE at LBA `ssd_size`).
- Verified: read.
- Port's handling: `sys/dev/softraid_concat.rs:45-48` answers EIO.
- Severity: crash.
- Fix: `>=` in `sr_validate_io`.

### EXT-85: ugen's isochronous ring pointer wraps before the buffer

- Where: `sys/dev/usb/ugen.c:863`, `ugen_isoc_rintr`.
- What: when the ring is full the oldest input is dropped (860-862) and `cur` wraps with
  `sce->cur = sce->ibuf + (sce->limit - sce->cur);` (863). That is zero or negative, so `cur`
  lands before `ibuf`. `ugenread` then sees `fill > cur` (611) and
  `uiomove(sce->cur, n, uio)` (619) copies the bytes before the ring;
  `filt_ugenread_isoc` (1279-1282) counts them.
- Reach: device. An isochronous IN endpoint read through `/dev/ugenN.EE` faster than the
  reader drains it (a webcam or audio device with a slow reader).
- Verified: read.
- Port's handling: `sys/dev/usb/ugen.rs:74` wraps with `cur - limit`.
- Severity: memory corruption (an out-of-bounds read: kernel heap copied to userland).
- Fix: `sce->cur = sce->ibuf + (sce->cur - sce->limit);`.

### EXT-86: a failed isochronous open frees transfers still queued

- Where: `sys/dev/usb/ugen.c:393`, `ugenopen`.
- What: each request is started with `(void)usbd_transfer(xfer);` (389). If a later
  allocation fails, `bad:` runs `while (--i >= 0) usbd_free_xfer(...)` (393-395) with the pipe
  still open, freeing DMA buffers and xfers the controller is using. The pipe and `ibuf`
  leak.
- Reach: alloc-failure (DMA memory exhausted after the first request).
- Verified: read.
- Port's handling: same fault (`sys/dev/usb/ugen.rs:617-627`; "Bugs the port reproduces").
- Severity: memory corruption.
- Fix: abort and close the pipe before freeing the xfers, and free `ibuf`.

### EXT-87: a CSW residue larger than the transfer gives `memcpy` a negative size

- Where: `sys/dev/usb/umass.c:1274`, `umass_bbb_state`.
- What: `sc->transfer_actlen = sc->transfer_datalen - UGETDW(sc->csw.dCSWDataResidue);`
  (1274-1275), then `memcpy(sc->transfer_data, sc->data_buffer, sc->transfer_actlen);`
  (1276-1277). The only residue check is under `#if 0` (1239-1247).
- Reach: device (a umass device answering a CSW whose residue exceeds the length).
- Verified: read.
- Port's handling: `sys/dev/usb/umass.rs:112-114` bounds the copies.
- Severity: memory corruption.
- Fix: reject a residue larger than `transfer_datalen`.

### EXT-88: the configuration descriptor's `wTotalLength` is trusted after the read

- Where: `sys/dev/usb/usb_subr.c:667`, `usbd_set_config_index` (also 644, 1409-1413).
- What: `cdp = malloc(cdplen)` with the header's length (667-668); the full read (673)
  overwrites `wTotalLength` and nothing compares it. `free(dev->cdesc, M_USB,
  UGETW(dev->cdesc->wTotalLength))` (644) then panics under `DIAGNOSTIC`, the walkers read
  past the buffer, and `usbd_get_cdesc` copies the excess to userland.
- Reach: device (a device whose two reads disagree; the panic comes at detach).
- Verified: read.
- Port's handling: `sys/dev/usb/usb_subr.rs:93-94` frees with the allocated size, and the
  walkers clamp.
- Severity: crash.
- Fix: refuse the configuration unless the lengths match and are at least the header.

### EXT-89: short interface and endpoint descriptors are accepted

- Where: `sys/dev/usb/usb_subr.c:533`, `usbd_parse_idesc` (and `usbd_find_idesc`, 429-433).
- What: an endpoint is accepted when `p + ed->bLength <= end && ed->bLength != 0` (533), even
  below 7 bytes. On a high-speed device `USETW(ed->wMaxPacketSize, mps);` (561-562) then
  writes 1-2 bytes past the buffer when the short descriptor ends it.
- Reach: device (a malicious high-speed device, at plug-in).
- Verified: read.
- Port's handling: `sys/dev/usb/usb_subr.rs:82-84` skips short descriptors.
- Severity: memory corruption.
- Fix: require `bLength >=` the structure's size.

### EXT-90: `usbd_get_no_alts` loops on a zero-length descriptor

- Where: `sys/dev/usb/usbdi.c:707`, `usbd_get_no_alts`.
- What: `for (n = 0; p < end; p += d->bLength)` (707) has no zero check.
- Reach: device. cdce, urndis, umb and uvideo call it at attach, and ugen's
  `USB_GET_NO_ALT` calls it too.
- Verified: read.
- Port's handling: `sys/dev/usb/usbdi.rs:118-119` stops.
- Severity: crash (a kernel hang).
- Fix: `if (d->bLength == 0) break;`.

### EXT-91: `usbd_get_hid_descriptor` loops on a zero-length descriptor

- Where: `sys/dev/usb/usbdi_util.c:218`, `usbd_get_hid_descriptor`.
- What: `for (; p < end; p += hd->bLength)` (218) has no zero check.
- Reach: device (a HID device, at attach).
- Verified: read.
- Port's handling: `sys/dev/usb/usbdi_util.rs:97` stops.
- Severity: crash (a kernel hang).
- Fix: break on `bLength == 0`.

### EXT-92: ohci divides by a zero `wMaxPacketSize`

- Where: `sys/dev/usb/ohci.c:525` (and 553), `ohci_alloc_std_chain`.
- What: `curlen -= curlen % mps;` (525) and `alen % mps == 0` (553), with `mps` from the
  endpoint (508). uhci refuses 0 (`uhci.c:1544-1547`).
- Reach: device (a full-speed device on OHCI with a zero bulk packet size).
- Verified: read.
- Port's handling: panics instead (`sys/dev/usb/ohci.rs:100`).
- Severity: crash.
- Fix: refuse `mps == 0` as uhci does.

### EXT-93: ohci bulk start uses the tail before checking the error

- Where: `sys/dev/usb/ohci.c:2761`, `ohci_device_bulk_start`.
- What: `err = ohci_alloc_std_chain(sc, len, xfer, data, &tail);` (2759), then
  `tail->td.td_flags &= ...` (2761) before `if (err) return (err);` (2765). On failure
  `tail` was never set.
- Reach: alloc-failure (an OHCI bulk or control transfer when TD allocation fails).
- Verified: read.
- Port's handling: `sys/dev/usb/ohci.rs:94-99` checks first.
- Severity: crash.
- Fix: check `err` before touching `tail`, and free the partial chain.

### EXT-94: xhci indexes arrays with ids from event TRBs

- Where: `sys/dev/usb/xhci.c:839`, `xhci_event_xfer` (also 1309, 1197).
- What: `if (slot > sc->sc_noslot)` (839) admits slot 128 for `sc_sdevs[128]`; `pipes[dci -
  1]` (844) with `dci` 0 reads `pipes[-1]`; `p[port/8] |= ...` (1197) uses a TRB port
  number past the status buffer.
- Reach: firmware/hardware (a controller with 128 slots, or one misbehaving).
- Verified: read.
- Port's handling: `sys/dev/usb/xhci.rs:67-70` ignores such ids.
- Severity: memory corruption.
- Fix: `slot >= sc_noslot`, and range-check `dci` and `port`.

### EXT-95: a UAC 2.0 clock chain ending in NULL is dereferenced

- Where: `sys/dev/usb/uaudio.c:1029`, `uaudio_alt_getrates`.
- What: `u = ... sc->pclock : sc->rclock; while (1) { switch (u->type) { ... case
  UAUDIO_AC_CLKSEL: u = u->clock;` (1027-1034) never checks `u`, unlike `uaudio_clock`
  (1057).
- Reach: device (a malicious UAC 2.0 device, when /dev/audio is opened).
- Verified: read.
- Port's handling: `sys/dev/usb/uaudio.rs:96-97` returns no rates.
- Severity: crash.
- Fix: `if (u == NULL) return 0;`.

### EXT-96: more than 8 channels are written past `level[8]`

- Where: `sys/dev/usb/uaudio.c:4432`, `uaudio_get_port_do` (and `uaudio_set_port_do`,
  4474-4480).
- What: `for (i = 0; i < nch; i++) ... ctl->un.value.level[i] = ...` (4417-4434), where
  `level` is `u_char level[8]` (`sys/sys/audioio.h:111`) and `nch` comes from the device.
- Reach: device (a device with more than 8 per-channel controls; then suspend, a volume
  key or mixerctl).
- Verified: read.
- Port's handling: `sys/dev/usb/uaudio.rs:101-102` uses 8 at most.
- Severity: memory corruption.
- Fix: clamp `nch` to `nitems(level)`.

### EXT-97: unnamed features reach `strcmp(NULL)`

- Where: `sys/dev/usb/uaudio.c:1170`, `uaudio_feature_addent` (and 1953).
- What: the table has `{NULL, -1, -1}` for delay, underflow and overflow (1119, 1125-1126),
  `m->fname = features[uac_type].name;` (1138) stores NULL, and `strcmp(i->fname,
  m->fname)` (1170) dereferences it.
- Reach: device (a feature unit advertising Delay with any other control).
- Verified: read.
- Port's handling: `sys/dev/usb/uaudio.rs:90-92` compares as empty.
- Severity: crash.
- Fix: skip entries without a name.

### EXT-98: the unique-name list keeps a dead stack buffer

- Where: `sys/dev/usb/uaudio.c:726`, `uaudio_mkname`.
- What: `n->templ = templ;` (726) stores the caller's stack array from
  `uaudio_setname_middle`, and later calls `strcmp` against it (733).
- Reach: device (any UAC device with two named feature units).
- Verified: read.
- Port's handling: `sys/dev/usb/uaudio.rs:86-89` compares copies.
- Severity: cosmetic (wrong control names, from an out-of-scope read).
- Fix: store a copy of the template.

### EXT-99: an unknown class version leaves locals uninitialised

- Where: `sys/dev/usb/uaudio.c:2045`, `uaudio_process_header`.
- What: `uaudio_getnum(&ph, 2, &sc->version)` (2045) takes any `bcdADC`, and the `switch
  (sc->version)` statements have no default: `count` and `p` in `uaudio_req_ranges`
  (945-1001), and `size` in the feature parse (1476-1482), are used uninitialised.
- Reach: device (a UAC 3.0 configuration or a malicious device).
- Verified: read.
- Port's handling: `sys/dev/usb/uaudio.rs:93-95` fails those parses.
- Severity: crash.
- Fix: refuse versions other than 1.0 and 2.0.

### EXT-100: a zero-length report wraps the length

- Where: `sys/dev/usb/uhidev.c:530`, `uhidev_intr`.
- What: `rep = *p++, cc--;` (530) with `cc == 0` wraps it, and `scd->sc_intr(scd, p, cc)`
  (543) hands that to children that copy it (`uslhcom.c:495`, `uhidpp.c:506`).
- Reach: device (a zero-length packet from a device with several report ids).
- Verified: read.
- Port's handling: `sys/dev/usb/uhidev.rs:84-87` clamps it.
- Severity: memory corruption.
- Fix: `if (cc == 0) return;`.

### EXT-101: `uhidev_set_report_async` takes `USBD_IN_PROGRESS` as failure

- Where: `sys/dev/usb/uhidev.c:819`, `uhidev_set_report_async`.
- What: `if (usbd_transfer(xfer)) { usbd_clear_endpoint_stall_async(sc->sc_opipe); actlen =
  -1; }` (819-826). A started async transfer returns `USBD_IN_PROGRESS` (1), so every
  success clears the stall under the transfer and reports -1. A real failure leaks the
  xfer.
- Reach: device (keyboard LEDs, `ugold`, on devices with an interrupt OUT endpoint).
- Verified: read.
- Port's handling: same fault (`sys/dev/usb/uhidev.rs:107`, `usbdi.rs:224`).
- Severity: wrong result.
- Fix: treat only errors other than `USBD_IN_PROGRESS` as failures, and free the xfer then.

### EXT-102: hub port bits above 31 are shifted out of an int

- Where: `sys/dev/usb/uhub.c:324` (and 370-371, 500-501).
- What: `sc->sc_status |= (1 << port);` (324) with up to 255 ports and a 32-bit status.
- Reach: device (a hub declaring 32 ports or more).
- Verified: read.
- Port's handling: `sys/dev/usb/uhub.rs:69` uses checked shifts.
- Severity: wrong result.
- Fix: a bitmap of `nports + 1` bits, or refuse such hubs.

### EXT-103: uftdi's 2232H rate divides by a speed of 0

- Where: `sys/dev/usb/uftdi.c:1223`, `uftdi_2232h_getrate`.
- What: `int n = (FTDI_2232H_FREQ << 3) / speed;` (1223) and `/ speed` (1239) with no check;
  `uftdi_8u232am_getrate` checks `speed <= 0` (1165). `uftdi_param` passes `c_ospeed`
  through (1038-1040).
- Reach: user (group dialer). `stty -f /dev/cuaU0 0` on an FT2232H or FT4232H (`bcdDevice`
  0x0700 or 0x0800, 794-796). `ttioctl` rejects only negative speeds (`tty.c:878`), and
  `ucomparam` (`ucom.c:892`) passes B0. amd64 traps (#DE); arm64's `udiv` gives 0.
- Verified: read.
- Port's handling: `sys/dev/usb/uftdi.rs:69` answers EINVAL.
- Severity: crash.
- Fix: `if (speed == 0) return -1;`.

### EXT-104: `uftdi_read` wraps the count on a packet shorter than two bytes

- Where: `sys/dev/usb/uftdi.c:949`, `uftdi_read`.
- What: `*ptr += 2; *count -= 2;` (948-949) with no length check, and `ucomreadcb` then
  feeds about 4 GB from past the buffer to the tty (`ucom.c:1193-1195`).
- Reach: device (a short bulk-IN packet while the port is open).
- Verified: read.
- Port's handling: `sys/dev/usb/uftdi.rs:72` advances a slice.
- Severity: crash (and kernel memory to the tty reader).
- Fix: `if (*count < 2) { *count = 0; return; }`.

### EXT-105: the ucom open failure frees the HID device's transfer

- Where: `sys/dev/usb/ucom.c:551`, `ucom_do_open`.
- What: for a HID port (uslhcom, ucycom) the xfers are uhidev's (408-410). If
  `usbd_alloc_buffer(sc->sc_oxfer, ...)` fails (413-417), `fail_4` skips the output xfer
  for HID ports but falls into `fail_3: usbd_free_xfer(sc->sc_ixfer);` (550-551). That frees
  uhidev's input xfer, whose interrupt transfer has been in flight since attach. `fail_2`
  and `fail_1` then close NULL pipes. Separately, a HID device without an output endpoint
  has no `sc_oxfer` (`uhidev.c:600, 614`), and 413 dereferences NULL (`usbdi.c:378`).
- Reach: alloc-failure (`BUS_DMA_NOWAIT`, `usb_mem.c:121-136`). The NULL case needs a
  device without that endpoint.
- Verified: read.
- Port's handling: `sys/dev/usb/ucom.rs:76` frees only what ucom allocated.
- Severity: memory corruption (use after free, then a double free at `uhidev_close`).
- Fix: guard `fail_3` to `fail_1` with `sc_bulkin_no != -1`, as `fail_4` does.

### EXT-106: hidkbd keeps a NULL `sc_var`, and uninitialised entries

- Where: `sys/dev/hid/hidkbd.c:737`, `hidkbd_parse_desc`.
- What: `sc_nvar = ivar; sc_var = mallocarray(..., M_NOWAIT); if (!kbd->sc_var) return
  NULL;` (733-738). NULL is the success return, so `hidkbd_input` indexes NULL. Entries
  skipped for multi-bit items (752-763) stay uninitialised.
- Reach: device (a crafted keyboard descriptor) or alloc-failure (the NULL case).
- Verified: read.
- Port's handling: `sys/dev/hid/hidkbd.rs:91` allocates zeroed, with no NULL left behind.
- Severity: crash.
- Fix: return an error string, use `M_ZERO`, and set `sc_nvar` to the entries filled.

### EXT-107: Wacom pad buttons are copied from the wrong index

- Where: `sys/dev/hid/hidms.c:233`, `hidms_wacom_setup`.
- What: `memcpy(&ms->sc_loc_btn[i], &loc_pad_btn[i], ...)` (232-234), with `i` starting at
  the stylus button count, reads pad button `num_stylus + k` from uninitialised stack
  entries.
- Reach: device (a Wacom tablet with stylus and pad buttons; real hardware).
- Verified: read.
- Port's handling: same fault on a zeroed array (`sys/dev/hid/hidms.rs:69-72`).
- Severity: wrong result.
- Fix: `&loc_pad_btn[i - ms->sc_num_stylus_buttons]`.

### EXT-108: `INT_MIN / -1` in the hidms calibration

- Where: `sys/dev/hid/hidms.c:584`, `hidms_input`.
- What: `dx = ((dx - minx) * resx) / (maxx - minx);` (584-587), where
  `WSMOUSEIO_SCALIBCOORDS` only refuses a zero divisor; -1 with an X of `0x80000000` traps on
  amd64.
- Reach: device (a calibration with `maxx = minx - 1`, plus a device sending that value).
- Verified: read.
- Port's handling: same fault (`sys/dev/hid/hidms.rs:678` panics).
- Severity: crash.
- Fix: compute in `int64_t`, or require `maxx > minx`.

### EXT-109: an nvme completion's command id indexes `sc_ccbs` unchecked

- Where: `sys/dev/ic/nvme.c:1218`, `nvme_q_complete`.
- What: `ccb = &sc->sc_ccbs[cqe->cid];` (1218) with a 16-bit id from the controller, then
  written and completed.
- Reach: firmware/hardware.
- Verified: read.
- Port's handling: panics instead (`sys/dev/ic/nvme.rs:63-67`).
- Severity: memory corruption.
- Fix: skip and report a `cid` out of range.

### EXT-110: a completion after `nvme_poll` timed out writes through the restored cookie

- Where: `sys/dev/ic/nvme.c:1148`, `nvme_poll`.
- What: on timeout only `ccb->ccb_cookie = cookie;` (1148) is restored, and `ccb_done`
  stays `nvme_poll_done` (1136). A late completion runs `state->c.flags = ...` (1168)
  through the caller's cookie: an `xs`, or a dead stack frame.
- Reach: firmware/hardware (a slow controller).
- Verified: read.
- Port's handling: `sys/dev/ic/nvme.rs:68-72` ignores a foreign cookie.
- Severity: memory corruption.
- Fix: restore `ccb_done` too, or abort the command.

### EXT-111: nvme passthrough copies out `pt_statuslen` bytes of stack

- Where: `sys/dev/ic/nvme.c:1014`, `nvme_passthrough_cmd`.
- What: `copyout(&pt_status, pt->pt_status, pt->pt_statuslen);` (1014) from a 20-byte local
  with the caller's length.
- Reach: root (`NVME_PASSTHROUGH_CMD` through /dev/bio or the sd raw device).
- Verified: read.
- Port's handling: `sys/dev/ic/nvme.rs:81-82` copies 20 bytes at most.
- Severity: leak (kernel stack disclosure).
- Fix: `MIN(pt->pt_statuslen, sizeof(pt_status))`.

### EXT-112: nvme's maximum transfer size overflows

- Where: `sys/dev/ic/nvme.c:1281`, `nvme_identify`.
- What: `sc->sc_mdts = (1 << identify->mdts) * (1 << mpsmin);` (1281-1286) overflows for
  MDTS of 20 or more (allowed by the spec), giving 0.
- Reach: firmware/hardware.
- Verified: read.
- Port's handling: `sys/dev/ic/nvme.rs:76-78` saturates.
- Severity: wrong result.
- Fix: compute in 64 bits and clamp to `NVME_MAXPHYS`.

### EXT-113: the PRP-list allocation is used unchecked

- Where: `sys/dev/ic/nvme.c:1425`, `nvme_ccbs_alloc`.
- What: `sc_ccb_prpls = nvme_dmamem_alloc(...); prpl = NVME_DMA_KVA(sc->sc_ccb_prpls);`
  (1422-1425) dereferences a NULL return.
- Reach: alloc-failure.
- Verified: read.
- Port's handling: `sys/dev/ic/nvme.rs:76` fails.
- Severity: crash.
- Fix: check for NULL.

### EXT-114: nvme's `rp` indexes a 4-entry array

- Where: `sys/dev/ic/nvme.c:2131`, `nvme_bioctl_disk`.
- What: `rpdesc[idns->lbaf[i].rp]` (2131) with `rpdesc[4]` (2064) and the whole byte as
  index.
- Reach: firmware/hardware (reserved bits set; BIOCDISK).
- Verified: read.
- Port's handling: `sys/dev/ic/nvme.rs:76-80` checks it.
- Severity: crash.
- Fix: `rp & 0x3`.

### EXT-115: `sc_q` dangles after a failed resume

- Where: `sys/dev/ic/nvme.c:1390`, `nvme_q_delete` and `nvme_resume`.
- What: `nvme_q_free(sc, q);` (1390) leaves `sc->sc_q` pointing at freed memory, and a failed
  resume never replaces it; I/O and `nvme_intr` (1586) then use it.
- Reach: firmware/hardware (a controller that does not come back on resume).
- Verified: read.
- Port's handling: `sys/dev/ic/nvme.rs:83-84` clears it.
- Severity: memory corruption (use after free).
- Fix: clear `sc_q` and check it.

### EXT-116: atascsi UNMAP completes the xfer and carries on

- Where: `sys/dev/ata/atascsi.c:1044` (and 1066), `atascsi_disk_unmap`.
- What: `atascsi_done(xs, XS_DRIVER_STUFFUP);` (1045) and `atascsi_done(xs, XS_NOERROR);`
  (1068) have no `return`, so the xfer is completed twice or a TRIM starts for it.
- Reach: root (`SCIOCCOMMAND` UNMAP on a raw ATA disk opened for writing).
- Verified: read.
- Port's handling: `sys/dev/ata/atascsi.rs:78-82` returns.
- Severity: memory corruption (a double completion).
- Fix: `return;` after both.

### EXT-117: atascsi's unmap failure frees `xa->data`, not its buffer

- Where: `sys/dev/ata/atascsi.c:1135`, `atascsi_disk_unmap_task`.
- What: `trims = dma_alloc(512, ...)` (1101); a range too long does `goto fail` (1110-1111),
  and `fail:` does `dma_free(xa->data, 512);` (1135) before `xa->data = trims` (1117).
- Reach: root (UNMAP with a descriptor over 65535 blocks).
- Verified: read.
- Port's handling: `sys/dev/ata/atascsi.rs:80-82` frees its own buffer.
- Severity: memory corruption (a foreign pointer freed).
- Fix: `dma_free(trims, 512);`.

### EXT-118: REQUEST SENSE with no data buffer writes through NULL

- Where: `sys/dev/ata/atascsi.c:1503`, `atascsi_disk_sense` (and `atascsi_pmp_sense`).
- What: `bzero(xs->data, xs->datalen); sd->error_code = ...; sd->flags = ...;` (1503-1506)
  with `xs->data` NULL when `datalen` is 0 (`scsi_ioctl.c:118-125`).
- Reach: root (operator group: REQUEST SENSE needs only read access, `scsi_ioctl.c:58`).
- Verified: read.
- Port's handling: `sys/dev/ata/atascsi.rs:83-85` writes within `datalen`.
- Severity: crash.
- Fix: write the fields only when `datalen` covers them.

### EXT-119: siop trusts the DSA register and the reselection tag

- Where: `sys/dev/ic/siop.c:374`, `siop_intr` (also 700, 707).
- What: `siop_cmd = &cbdp->cmds[dsa / sizeof(struct siop_xfer)];` (374) for any DSA in the
  page reaches past `cmds[]`, and `siop_cmd->cmd_c.xs->sc_link` (383) follows a NULL `xs`.
  A target's tag indexes `siop_tag[16]` unchecked.
- Reach: firmware/hardware (the chip or a target misbehaving).
- Verified: read.
- Port's handling: `sys/dev/ic/siop.rs:75-78`.
- Severity: memory corruption.
- Fix: range-check the index, `xs` and the tag.

### EXT-120: ahci indexes `ap_ccbs` with slot numbers from the controller and the disk

- Where: `sys/dev/ic/ahci.c:2263` (and 2630-2633), `ahci_port_intr`,
  `ahci_port_read_ncq_error`.
- What: `slot = AHCI_PREG_CMD_CCS(...); ccb = &ap->ap_ccbs[slot];` (2259-2264), and the NCQ
  log's tag, both 5-bit, index `sc_ncmds` entries.
- Reach: firmware/hardware.
- Verified: read.
- Port's handling: panics instead (`sys/dev/ic/ahci.rs:80`).
- Severity: memory corruption.
- Fix: reject `slot >= sc_ncmds`.

### EXT-121: a virtio capability's BAR number indexes a 6-entry stack array

- Where: `sys/dev/pci/virtio_pci.c:478`, `virtio_pci_attach_10`.
- What: `int bar = caps[i]->bar; ... if (bars[bar] < len) bars[bar] = len;` (474-479) with
  `bars[NMAPREG]` (449) and an 8-bit field from the device.
- Reach: firmware/hardware (a hypervisor or emulated device).
- Verified: read.
- Port's handling: `sys/dev/pci/virtio_pci.rs:64-67` refuses it.
- Severity: memory corruption (a stack write).
- Fix: reject `bar >= NMAPREG`.

### EXT-122: vioblk and vioscsi dereference a NULL transfer

- Where: `sys/dev/pv/vioblk.c:344` (and `vioscsi.c:299-305`), `vioblk_vq_done1`.
- What: `struct scsi_xfer *xs = vr->vr_xs;` (344) is used unchecked, for a slot the used
  ring names (`virtio_dequeue` does not bound it).
- Reach: firmware/hardware (a hypervisor completing an empty slot).
- Verified: read.
- Port's handling: panics instead (`sys/dev/pv/vioblk.rs:98-99`, `vioscsi.rs:65-66`).
- Severity: crash.
- Fix: ignore such completions.

### EXT-123: vmx uses an uninitialised interrupt handle

- Where: `sys/dev/pci/if_vmx.c:313`, `vmxnet3_attach`.
- What: `if (pci_intr_map_msix(pa, 0, &ih) == 0) {...} break;` (313-323) breaks out even
  when the map failed, and `pci_intr_establish(..., ih, ...)` (339-341) uses the garbage.
- Reach: firmware/hardware (MSI-X advertised but unmappable, e.g. a VM without MP tables).
- Verified: read.
- Port's handling: `sys/dev/pci/if_vmx.rs:81-82` stops.
- Severity: wrong result.
- Fix: break only on success.

### EXT-124: vmx's RSS DMA allocation is used unchecked

- Where: `sys/dev/pci/if_vmx.c:569`, `vmxnet3_dma_init`.
- What: `rsscfg = vmxnet3_dma_allocmem(...); rsscfg->hash_type = ...` (569-571).
- Reach: alloc-failure.
- Verified: read.
- Port's handling: `sys/dev/pci/if_vmx.rs:82-84` fails.
- Severity: crash.
- Fix: `if (rsscfg == NULL) return -1;`.

### EXT-125: em's PHY workaround writes uninitialised values

- Where: `sys/dev/pci/if_em_hw.c:7781`, `em_phy_no_cable_workaround`.
- What: `em_read_phy_reg(hw, I2_DFT_CTRL, &phy_reg);` (7781) ignores failure and writes
  `phy_reg | (1 << 14)` back; four more registers are read-modify-written the same way.
- Reach: firmware/hardware (a PHY read failing on an I217/I218 reset).
- Verified: read.
- Port's handling: `sys/dev/pci/if_em_hw.rs:109` starts them at 0.
- Severity: wrong result.
- Fix: check each read.

### EXT-126: `ppb_hotplug_insert` with no pci bus behind the bridge

- Where: `sys/dev/pci/ppb.c:785`, `ppb_hotplug_insert`.
- What: `psc = (struct pci_softc *)sc->sc_psc; if (!LIST_EMPTY(&psc->sc_devs))` (783-785),
  where `sc_psc` is NULL when the child bus did not attach (364).
- Reach: root (`pci* at ppb?` disabled in UKC, then a hot-plug event).
- Verified: read.
- Port's handling: `sys/dev/pci/ppb.rs:77-78` returns.
- Severity: crash.
- Fix: `if (psc == NULL) return;`.

### EXT-127: uhci_pci and re_pci disestablish a stale interrupt twice

- Where: `sys/dev/pci/uhci_pci.c:222` (and `if_re_pci.c:209-211`).
- What: the attach failure path does `pci_intr_disestablish(sc->sc_pc, sc->sc_ih);` (222)
  without clearing `sc_ih`, and detach disestablishes it again (236-238); re_pci also
  unmaps twice.
- Reach: firmware/hardware (a failed attach, then a hot unplug).
- Verified: read.
- Port's handling: `sys/dev/pci/uhci_pci.rs:65`, `if_re_pci.rs:52` clear it.
- Severity: memory corruption (a double free of the cookie).
- Fix: `sc->sc_ih = NULL` on the failure paths.

### EXT-128: `ahci_pci_detach` disestablishes a NULL interrupt

- Where: `sys/dev/pci/ahci_pci.c:439`, `ahci_unmap_intr`.
- What: `pci_intr_disestablish(psc->psc_pc, sc->sc_ih);` (439) unconditionally, after an
  attach whose `ahci_map_intr` failed.
- Reach: firmware/hardware (then a hot unplug).
- Verified: read.
- Port's handling: `sys/dev/pci/ahci_pci.rs:56` skips it.
- Severity: crash.
- Fix: `if (sc->sc_ih != NULL)`.

### EXT-129: `re_init` acknowledges after starting the moderation timer

- Where: `sys/dev/ic/re.c:1998`, `re_init`.
- What: `re_setup_intr(sc, 1, sc->rl_imtype); CSR_WRITE_2(sc, RL_ISR, sc->rl_intrs);`
  (1998-1999). The timer's expiry, the only interrupt, can be cleared by the second write,
  and transmit then waits for the watchdog.
- Reach: firmware/hardware (timing; seen in QEMU's 8139C+, one arm64 boot in three).
- Verified: read (the timing is the port's observation).
- Port's handling: `sys/dev/ic/re.rs:109-118` acknowledges first.
- Severity: wrong result.
- Fix: write `RL_ISR` first.

### EXT-130: fxp copies an oversized microcode file past the download block

- Where: `sys/dev/ic/fxp.c:1847`, `fxp_load_ucode`.
- What: `for (i = 0; i < (sc->sc_ucodelen / sizeof(u_int32_t)); i++) cbp->ucode[i] =
  sc->sc_ucodebuf[i];` (1847-1848) copies the whole file into `ucode[MAXUCODESIZE]` (192
  dwords, `fxpreg.h:338-343`), the last member of the DMA control block.
- Reach: root (a corrupt or replaced `/etc/firmware/fxp-*`; the shipped files fit).
- Verified: read.
- Port's handling: `sys/dev/ic/fxp.rs:84` refuses it.
- Severity: memory corruption.
- Fix: refuse `sc_ucodelen > sizeof(cbp->ucode)`.

### EXT-131: dc(4)'s PCI softc is allocated with `struct dc_softc`'s size

- Where: `sys/dev/pci/if_dc_pci.c:513`, `dc_pci_ca`.
- What: `sizeof(struct dc_softc)` (512-513), but the driver uses `struct dc_pci_softc`
  (109-113), which adds `psc_pc` and `psc_mapsize`. Attach writes them (168, 181, 188) and
  detach reads them (495-507), past the allocation.
- Reach: firmware/hardware (every dc(4) PCI attach; whether malloc's rounding hides it
  depends on `sizeof(struct dc_softc)`).
- Verified: read.
- Port's handling: `sys/dev/pci/if_dc_pci.rs:67` uses the right size.
- Severity: memory corruption.
- Fix: `sizeof(struct dc_pci_softc)`.

### EXT-132: the Conexant MAC is read from the softc, not the SROM

- Where: `sys/dev/ic/dc.c:1609`, `dc_attach`.
- What: `bcopy(&sc->dc_srom + DC_CONEXANT_EE_NODEADDR, &sc->sc_arpcom.ac_enaddr,
  ETHER_ADDR_LEN);` (1609-1610). `&sc->dc_srom` is the pointer member's address, so `+
  0x19A` steps 410 pointers (3280 bytes on LP64).
- Reach: firmware/hardware (every Conexant RS7112).
- Verified: read.
- Port's handling: `sys/dev/ic/dc.rs:86` reads the SROM bytes.
- Severity: wrong result (a bogus MAC; the read may leave the softc).
- Fix: `sc->dc_srom + DC_CONEXANT_EE_NODEADDR`.

### EXT-133: the Macronix/PNIC II MAC offset is read into half an int

- Where: `sys/dev/ic/dc.c:1582`, `dc_attach`.
- What: `int mac_offset` (1568) gets one 16-bit word in its first two bytes (1582-1583), and
  `mac_offset / 2` (1585) uses all four. On big-endian machines the offset comes from the
  uninitialised half.
- Reach: firmware/hardware (98713, 98713A, 987x5 and PNIC II; wrong on big-endian).
- Verified: read.
- Port's handling: `sys/dev/ic/dc.rs:84` uses a zeroed 16-bit word.
- Severity: wrong result.
- Fix: read into a `u_int16_t`.

### EXT-134: dc detach unmaps one page of three

- Where: `sys/dev/ic/dc.c:3103`, `dc_detach`.
- What: `bus_dmamem_unmap(sc->sc_dmat, sc->sc_listkva, sc->sc_listnseg);` (3103) passes the
  segment count (1) as the size of a 10,496-byte mapping (1639-1640).
- Reach: firmware/hardware (detach: a CardBus eject or PCI detach). i386 and arm64 keep two
  pages mapped to freed memory; amd64's direct map makes it a no-op.
- Verified: read.
- Port's handling: `sys/dev/ic/dc.rs:100` unmaps the whole list.
- Severity: leak (kernel virtual space, with stale mappings).
- Fix: `sizeof(struct dc_list_data)`.

### EXT-135: the 21143 SROM parser trusts the SROM's offsets

- Where: `sys/dev/ic/dc.c:1512`, `dc_parse_21143_srom` (and `dc_apply_fixup`, 1388-1396).
- What: `loff = sc->dc_srom[27]; lhdr = (struct dc_leaf_hdr *)&(sc->dc_srom[loff]);`
  (1512-1513) and the block walks (1520-1556) never compare with `dc_sromsize` (128 bytes
  for a 6-bit EEPROM). A failed `M_NOWAIT` in `dc_read_srom` (1497-1499) leaves `dc_srom`
  NULL, dereferenced unchecked.
- Reach: firmware/hardware (a corrupt or unusual SROM; the NULL case is alloc-failure).
- Verified: read.
- Port's handling: `sys/dev/ic/dc.rs:92` bounds every SROM read.
- Severity: wrong result (an out-of-bounds read used as media setup).
- Fix: bound by `dc_sromsize`, and skip without an SROM.

### EXT-136: dc's PNIC receive workaround overruns its salvage buffer

- Where: `sys/dev/ic/dc.c:1990`, `dc_pnic_rx_bug_war`.
- What: the loop (1990-2001) copies one buffer per descriptor into a 5-buffer area
  (`if_dc_pci.c:351`) without a count; `while(*ptr == 0x00) ptr--;` (2007-2008) has no lower
  bound; `bcopy(ptr, mtod(m, char *), total_len);` (2025) copies the chip's length into a
  cluster.
- Reach: firmware/hardware (an 82c168/169 whose receive bug spans more than five
  descriptors).
- Verified: read.
- Port's handling: `sys/dev/ic/dc.rs:96` bounds all three.
- Severity: memory corruption.
- Fix: stop after five buffers, bound the scan, clamp `total_len`.

### EXT-137: dc's receive trusts the descriptor's frame length

- Where: `sys/dev/ic/dc.c:2153`, `dc_rxeof`.
- What: `total_len = DC_RXBYTES(rxstat);` (2101; 14 bits) and `m_devget(mtod(m, char *),
  total_len, ETHER_ALIGN)` (2153) out of one cluster, whose buffer is 1536 bytes (1910) at
  8 bytes in (1892). Frames flagged only `GIANT` are accepted (2129-2131), and
  `FIRSTFRAG`/`LASTFRAG` are not checked outside the PNIC path.
- Reach: remote (a frame longer than 1536 bytes from the wire). Its length is reported in
  the last descriptor and read from that one cluster; past 2040 bytes the copy leaves it.
  How long a frame the chip accepts depends on its receive watchdog (not checked).
- Verified: read (the chip's maximum frame length: claimed).
- Port's handling: `sys/dev/ic/dc.rs:96` bounds the copy by the cluster.
- Severity: memory corruption (an out-of-bounds read of the cluster pool into a packet).
- Fix: drop descriptors without both FIRSTFRAG and LASTFRAG, or longer than the buffer.

### EXT-198: a timed-out mpi_poll leaves a dangling reply pointer

- `sys/dev/ic/mpi.c:1230` (3ce1f3f79392): `mpi_poll` points `ccb_cookie` at its local `rv`
  and returns on timeout with the ccb still queued; a late completion writes through the
  pointer into a dead stack frame. Port: the result lives in the ccb (`ccb_poll_rv`), so a
  late reply writes there (Deviations, `sys/dev/ic/mpi.rs`).

### EXT-199: pcscp stores the FIFO's leftover byte before checking the count

- `sys/dev/pci/pcscp.c:466`: `p += trans; *p = ...` runs before `if (trans < 0)` at line 471,
  so a negative count writes before the buffer. pcscp is deferred to M17; the partial port on
  its M17 branch stores the byte only inside the buffer.

### EXT-200: ncr53c9x takes a reselection with no target bit

- `sys/dev/ic/ncr53c9x.c:1341`: a `selid` of 0 passes the one-bit check and gives target
  `ffs(0) - 1`, which indexes `sc_tinfo` out of bounds. Deferred to M17; the partial port treats
  it as an invalid selid (DEVICE RESET).

### EXT-201: wdc's odd-length PIO input writes past the buffer

- `sys/dev/ic/wdc.c:1490`, `wdc_input_bytes`: the tail is rounded up to a whole word and read
  into the buffer, one byte past an odd-length buffer (`wdc_output_bytes`, line 1465, reads one
  past). Port: the last word goes through a two-byte bounce (`sys/dev/ic/wdc.rs`).

### EXT-202: sdmmc_io_rw_direct releases the bus lock twice

- `sys/dev/sdmmc/sdmmc_io.c:373`: on a `sdmmc_select_card` failure it calls `rw_exit` on
  `sc_lock`, which every caller holds and releases again. Port: kept as the C.

### EXT-203: pciide's chip maps use an unchecked M_NOWAIT cookie

- `sys/dev/pci/pciide.c:4286`, and 4559, 4888, 5146, 6806, 7503: `sc_cookie` from
  `malloc(..., M_NOWAIT | M_ZERO)` is used without a NULL check. Port: a failed allocation
  panics (Deviations, `sys/dev/pci/pciide.rs`).

### EXT-204: ufshci frees the request list the running controller uses

- `sys/dev/ic/ufshci.c:1429`: when `ufshci_ccb_alloc` fails, `ufshci_ccb_free` frees the
  transfer request list while UTRLBA still points at it and the list runs, and leaks the
  other two DMA areas. Deferred to M17 (the partial port keeps the C).

### EXT-205: mpi leaks the ccb when mpi_poll fails

- `sys/dev/ic/mpi.c:816` (`mpi_portenable`), 2585 and 2647 (`mpi_req_cfg_header`,
  `mpi_req_cfg_page`), 3055 (`mpi_ioctl_cache`): the error return leaves the ccb unreturned.
  Port: kept as the C.

### EXT-206 to EXT-211: pciide chip slips

- EXT-206, `sys/dev/pci/pciide.c:4217` and 4224 (`cmd680_setup_channel`): the DMA and PIO
  timings write their low and high bytes to the same `off`, so the high byte wins.
- EXT-207, `sys/dev/pci/pciide.c:6088` (`hpt_chip_map`): the channel loop sets `compatchan` to
  0 (then `i`), overwriting the value taken from `pa_function` at line 6064.
- EXT-208, `sys/dev/pci/pciide.c:5018` (`cy693_setup_channel`): `dma_mode` -1 is stored into
  the `u_int8_t` `DMA_mode` of both drives (255) before it is clamped to 0 at line 5022.
- EXT-209, `sys/dev/pci/pciide.c:9023` (`rdc_setup_channel`): `patr &= EN(0) | EN(1)` clears the
  other channel's timings, and the function neither sets the DMA status bits nor prints the
  modes as the other chips do.
- EXT-210, `sys/dev/pci/pciide.c:1703` (`pciide_unmapregs_compat`): the control registers are
  unmapped through `cmd_ioh`.
- EXT-211, `sys/dev/pci/pciide.c:1787` (`pciide_mapregs_native`): the error path unmaps
  `ctl_baseioh` with `cmd_iot`.
- Port: all kept as the C (`sys/dev/pci/pciide.rs`).

### EXT-212, EXT-213: sdmmc loses a failure

- EXT-212, `sys/dev/sdmmc/sdmmc_mem.c:655` (`sdmmc_mem_init`): `error = 1` from a failed select
  or block-length change is overwritten by the next line.
- EXT-213, `sys/dev/sdmmc/sdmmc_mem.c:917` (`sdmmc_mem_mmc_init`): a failed bus-width change is
  overwritten by the clock change's result.
- Port: both kept as the C.

### EXT-214: wd_flushcache's ENODEV never happens

- `sys/dev/ata/wd.c:1065`: `wdc_c.flags & ERR_NODEV` tests an `ata_bio` error code (5) as an
  `AT_*` command flag. Port: kept as the C.

### EXT-215: pcscp's attach error path unmaps the wrong pointer

- `sys/dev/pci/pcscp.c:304` (`fail_2`): `bus_dmamem_unmap` is given `sc_mdldmap` (the DMA map)
  instead of `sc_mdladdr`. Deferred to M17; the partial port unmaps the mapping.

### EXT-216: ncr53c9x reads sc_imess past its end

- `sys/dev/ic/ncr53c9x.c:1454` prints `sc_imess[sc_imlen]` after a dropped byte, with
  `sc_imlen` possibly 9 or more, and line 2481 reads `sc_imess[sc_imlen - 2]` when `sc_imlen`
  may be below 2. Deferred to M17; the partial port reads 0 there.

### EXT-217: mpi's cache ioctl reads the rcb as the reply

- `sys/dev/ic/mpi.c:3060`: `rep = (struct mpi_msg_raid_action_reply *)ccb->ccb_rcb` casts
  the rcb, not `rcb_reply`, so `action_status` is not the reply's. Port: reads the same bytes.

### EXT-218: a failed wddump blocks every later dump

- `sys/dev/ata/wd.c:905`: `wddoingadump` is set before the checks that return ENXIO, EFAULT
  or EINVAL and is cleared only on success, so a second attempt gets EFAULT; the `wdlookup`
  reference is never released. Port: kept as the C.

### EXT-219: MAKEDEV numbers fd1's nodes as unit 0

- `etc/etc.amd64/MAKEDEV:606`: `n = U*128 + typnum*64`, but `FDUNIT` is
  `(minor / MAXPARTITIONSUNIT) / 8` (`sys/dev/isa/fdreg.h:67`), so unit 1 starts at minor
  512 and the `fd1*` nodes open unit 0 with another density. The ramdisk has only fd0's.

### EXT-220, EXT-221: mpi's bio ioctls read past the inquiry fields

- EXT-220, `sys/dev/ic/mpi.c:3297` (`mpi_ioctl_vol`): `memcpy` of `sizeof bv_vendor` (32)
  bytes from the 8-byte inquiry vendor field.
- EXT-221, `sys/dev/ic/mpi.c:3348` (`mpi_ioctl_disk`): `strlcpy` from the 8-byte
  `vendor_id`, which is not NUL-terminated, runs into `product_id`.
- Port: both kept as the C.

### EXT-222: mpi_msg_eventack_reply's ioc_status width

- `sys/dev/ic/mpireg.h:754`: `ioc_status` is `u_int32_t` where every other reply has a
  16-bit one, so the structure is 2 bytes longer than the reply; nothing uses its size.
  Port: kept.

## ACPI, consoles, audio and platform

### EXT-138: `aml_compare` leaks the converted first operand

- Where: `sys/dev/acpi/dsdt.c:2164`, `aml_compare`.
- What: a non-Integer/String/Buffer `a1` is converted (`cv = aml_tryconv(a1, ...)`, 2164)
  and only the converted `a2` is released (2186).
- Reach: firmware/hardware (valid AML: `LEqual(DerefOf(Arg0), 1)` on a field reference; it
  leaks on every evaluation).
- Verified: read.
- Port's handling: `sys/dev/acpi/dsdt.rs:91` (reference counted).
- Severity: leak.
- Fix: release `cv`.

### EXT-139: `Create*Field` on a non-Buffer source leaks the converted buffer

- Where: `sys/dev/acpi/dsdt.c:2982`, `aml_createfield`.
- What: `data = aml_convert(data, AML_OBJTYPE_BUFFER, -1);` (2971) returns a new value with
  one reference, then `aml_addref(data, "Field.Data");` (2982) takes a second; only one is
  ever dropped.
- Reach: firmware/hardware (valid AML: `CreateDWordField(Arg0, ...)` on an Integer).
- Verified: read.
- Port's handling: `sys/dev/acpi/dsdt.rs:91`.
- Severity: leak.
- Fix: do not add a reference to a just-converted buffer.

### EXT-140: a Return inside While leaks the scopes

- Where: `sys/dev/acpi/dsdt.c:1845`, `aml_findscope`/`aml_parse`.
- What: on Return a While scope gets `scope->pos = NULL;` (1844-1845). The unwinding loop
  `while (scope->pos >= scope->end && ...)` stops there, and `aml_eval` pops only the
  method's scope (3650).
- Reach: firmware/hardware (a common AML pattern; one scope per level per call).
- Verified: read.
- Port's handling: `sys/dev/acpi/dsdt.rs:433` (scopes are reference counted).
- Severity: leak.
- Fix: pop every scope above `iscope` after a Return.

### EXT-141: `Buffer(n){init}` copies the whole initializer

- Where: `sys/dev/acpi/dsdt.c:4117`, `aml_parse`.
- What: `aml_allocvalue(AML_OBJTYPE_BUFFER, opargs[0]->v_integer, NULL)` (4115-4116), then
  `memcpy(my_ret->v_buffer, opargs[1]->v_buffer, opargs[1]->length);` (4117-4118).
- Reach: firmware/hardware (a runtime size smaller than the initializer; ACPICA grows the
  buffer instead).
- Verified: read.
- Port's handling: `sys/dev/acpi/dsdt.rs:106` copies `n` bytes.
- Severity: memory corruption.
- Fix: allocate `max(n, initializer length)`.

### EXT-142: Buffer Fields are not bounds-checked

- Where: `sys/dev/acpi/dsdt.c:2925` (and 2930), `aml_rwfield`.
- What: `aml_bufcpy(&val->v_integer, 0, ref1->v_buffer, bitpos, bitlen);` (2925-2926) reads
  past the source buffer and writes `bitlen` bits into the 8-byte `v_integer`. The write
  path (2930-2931) writes past the buffer. `aml_createfield` never checks the field
  against its buffer.
- Reach: firmware/hardware (buggy AML, or a valid field wider than 64 bits).
- Verified: read.
- Port's handling: `sys/dev/acpi/dsdt.rs:108-111` bounds the copies.
- Severity: memory corruption.
- Fix: reject fields past their buffer, and read wide fields into a buffer.

### EXT-143: the AML parser reads past the end of a table

- Where: `sys/dev/acpi/dsdt.c:3986`, `aml_parse`.
- What: names, package lengths and immediates are read without checking `scope->end`, and
  `ch = (*end == AMLOP_ELSE && end < scope->end) ?` (3986) dereferences `end` before
  comparing it.
- Reach: firmware/hardware (a truncated table; the 1-byte read also happens with valid AML
  ending in an If).
- Verified: read.
- Port's handling: `sys/dev/acpi/dsdt.rs:105` reads zeros past the table.
- Severity: crash (only if the copy ends at a page boundary).
- Fix: compare before dereferencing, and bound the reads.

### EXT-144: `Ones` is 255 on arm64

- Where: `sys/dev/acpi/dsdt.c:4019`, `aml_parse`.
- What: `aml_allocvalue(AML_OBJTYPE_INTEGER, (char)opcode, NULL)` (4019-4020); arm64's
  `char` is unsigned, so `Ones` (0xFF) is 255, not -1.
- Reach: firmware/hardware (every arm64 ACPI machine whose AML uses `Ones`).
- Verified: read.
- Port's handling: `sys/dev/acpi/dsdt.rs:112-114` gives -1.
- Severity: wrong result.
- Fix: `(int8_t)opcode`.

### EXT-145: no DSDT dereferences NULL

- Where: `sys/dev/acpi/acpi.c:1189`, `acpi_attach_common`.
- What: `if (entry == NULL) printf(" !DSDT");` (1186-1187), then `p_dsdt =
  entry->q_table;` (1189).
- Reach: firmware/hardware (a missing or unmappable DSDT; or `M_NOWAIT` failing).
- Verified: read.
- Port's handling: `sys/dev/acpi/acpi.rs:79` parses nothing.
- Severity: crash.
- Fix: return after the message.

### EXT-146: a short FADT is read past its copy

- Where: `sys/dev/acpi/acpi.c:1276`, `acpi_attach_common`.
- What: `sc->sc_fadt->x_pm_tmr_blk.address` (1276; offset 208) is read with no revision
  check from a copy of `hdr->length` bytes (116 for revision 1).
- Reach: firmware/hardware (an old FADT without a PM timer).
- Verified: read.
- Port's handling: `sys/dev/acpi/acpi.rs:69-71` zero-pads the copy.
- Severity: wrong result.
- Fix: check `hdr_revision >= 3` as `acpitimer.c:83` does.

### EXT-147: the GAS Access Size code is used as a byte count

- Where: `sys/dev/acpi/acpi.c:1544`, `acpi_read_pmreg`/`acpi_write_pmreg` (also
  `acpicpu_x86.c:933`).
- What: the sleep registers pass `register_bit_width / 8, access_size, &value` to
  `acpi_gasio(..., access_size, len, buffer)` (1544-1548): the Access Size code (0-4)
  becomes the length, into a 1-byte `uint8_t value` (1541). `KASSERT((len % access_size) ==
  0)` (247) panics for conformant 32-bit registers; code 0 drops the access; codes 2-4
  overrun the byte. `_PCT` does the same (`acpicpu_x86.c:933, 936`).
- Reach: firmware/hardware (HW-reduced sleep registers; an x86 `_PCT` filled per the spec).
- Verified: read.
- Port's handling: same fault (`sys/dev/acpi/acpi.rs:75-76`, `acpicpu_x86.rs:85-86` only stop
  the overrun).
- Severity: crash (the KASSERT), memory corruption (the overrun).
- Fix: convert the code to bytes (`1 << (code - 1)`, 0 meaning the bit width).

### EXT-148: `acpicpu_setperf` divides by zero and indexes before `sc_pss`

- Where: `sys/dev/acpi/acpicpu_x86.c:1107`, `acpicpu_setperf`.
- What: `idx = (len - 1) - (level / (100 / len));` (1107) divides by zero for `len` 0 or
  over 100; `idx += sc->sc_pss_len - sc->sc_ppc;` (1111-1112) goes negative when `_PPC`
  exceeds the states, and `pss = &sc->sc_pss[idx];` (1120) writes that entry's control value
  to the chip.
- Reach: firmware/hardware (through `hw.setperf`, apmd, perfpolicy).
- Verified: read.
- Port's handling: `sys/dev/acpi/acpicpu_x86.rs:84-85` bounds it.
- Severity: crash.
- Fix: clamp `sc_ppc`, return for an empty list, avoid `100 / len`.

### EXT-149: a C4 `_CST` entry increments past `cst_stats[4]`

- Where: `sys/dev/acpi/acpicpu_x86.c:1212`, `acpicpu_idle`.
- What: `state > 4` is refused (385), so 4 is kept, and `atomic_inc_long(&cst_stats[best->
  state]);` (1212) writes past `cst_stats[4]` (115) on every idle entry.
- Reach: firmware/hardware (a non-conforming Type-4 entry on an ARAT CPU).
- Verified: read.
- Port's handling: `sys/dev/acpi/acpicpu_x86.rs:82-83` does not count it.
- Severity: memory corruption.
- Fix: size the array 5, or refuse `state > 3`.

### EXT-150: `_PSS` and `_PCT` are used without type or length checks

- Where: `sys/dev/acpi/acpicpu_x86.c:965`, `acpicpu_getpss` (and `acpicpu_getpct`, 875).
- What: `aml_val2int(res.v_package[i]->v_package[0])` (965) and `[1..5]` assume 6-element
  packages; `_PCT` copies `v_buffer` without checking its type or length (891, 899).
- Reach: firmware/hardware (at attach and on each 0x80 notify).
- Verified: read.
- Port's handling: `sys/dev/acpi/acpicpu_x86.rs:88-89`.
- Severity: crash.
- Fix: check types and lengths.

### EXT-151: the DMAR and IVRS walks loop on a zero-length structure

- Where: `sys/dev/acpi/acpidmar.c:2782` (and 2616, 3617), `acpidmar_init`.
- What: `off += de->length;` (2782) never checks for 0; a zero DRHD allocates and inits an
  IOMMU on each pass.
- Reach: firmware/hardware (acpidmar enabled through `boot -c`).
- Verified: read.
- Port's handling: `sys/dev/acpi/acpidmar.rs:114-115` stops.
- Severity: crash (a boot hang with memory exhaustion).
- Fix: stop on a short length or one past the table.

### EXT-152: acpiprt uses an uninitialised IRQ

- Where: `sys/dev/acpi/acpiprt.c:249`, `acpiprt_prt_add` (and 417).
- What: `struct acpiprt_irq irq;` (249) is set only by an IRQ or Extended IRQ descriptor;
  without one, `p->irq = irq._int` (319) and the routing use stack garbage.
- Reach: firmware/hardware (a link `_CRS` without an interrupt descriptor).
- Verified: read.
- Port's handling: `sys/dev/acpi/acpiprt.rs:55-56` zeroes it.
- Severity: wrong result.
- Fix: start `_int` at -1 and treat it as none.

### EXT-153: a bus number over 255 writes past `mp_busses`

- Where: `sys/dev/acpi/acpiprt.c:380`, `acpiprt_prt_add`.
- What: `map->next = mp_busses[sc->sc_bus].mb_intrs; mp_busses[sc->sc_bus].mb_intrs = map;`
  (380-381) into `acpimadt_busses[256]`, with a bus from `_CRS` or `_BBN` unchecked.
- Reach: firmware/hardware.
- Verified: read.
- Port's handling: `sys/dev/acpi/acpiprt.rs:57-58` skips it.
- Severity: memory corruption.
- Fix: skip buses `>= mp_nbusses`.

### EXT-154: acpimadt dereferences a NULL I/O APIC

- Where: `sys/dev/acpi/acpimadt.c:370` (and 457), `acpimadt_attach`.
- What: `apic = ioapic_find_bybase(pin);` (363) then `map->ioapic_pin = pin -
  apic->sc_apic_vecbase;` (370) with no NULL check; the ISA loop does the same (445-457).
- Reach: root (`boot -c`, `disable ioapic`: every PC MADT has overrides); also firmware.
- Verified: read.
- Port's handling: `sys/dev/acpi/acpimadt.rs:62-63` skips it.
- Severity: crash.
- Fix: skip and report a pin no I/O APIC covers.

### EXT-155: wsemul_dumb's `crippled` is uninitialised for non-console screens

- Where: `sys/dev/wscons/wsemul_dumb.c:102`, `wsemul_dumb_attach`.
- What: the state is `malloc`ed without `M_ZERO` (110) and every member but `crippled` is
  set; `wsemul_dumb_output` branches on it (136).
- Reach: root (`wsconscfg -e dumb` on a kernel with WSEMUL_DUMB, as on sparc64).
- Verified: read.
- Port's handling: `sys/dev/wscons/wsemul_dumb.rs:68-69` computes it.
- Severity: wrong result (a crash through a NULL emulop if the display is crippled).
- Fix: compute it as `wsemul_dumb_cnattach` does.

### EXT-156: the wsdisplay copy buffer leaks when its size is unchanged

- Where: `sys/dev/wscons/wsdisplay.c:3487`, `allocate_copybuffer`.
- What: the old buffer is freed only `if (size != sc->sc_copybuffer_size && ...)` (3483),
  then `sc->sc_copybuffer = malloc(size, ...)` (3487) overwrites it.
- Reach: root (each wsmoused start and each added screen; 2-8 KB each time).
- Verified: read.
- Port's handling: `sys/dev/wscons/wsdisplay.rs:141-143` frees it.
- Severity: leak.
- Fix: free any old buffer.

### EXT-157: a paste after reallocation reads stale memory

- Where: `sys/dev/wscons/wsdisplay.c:3441`, `mouse_paste`.
- What: the new buffer is not zeroed and `SC_PASTE_AVAIL` stays set, so `strlen(...)`
  (3441) runs over stale heap bytes, maybe past the buffer, into the tty's input.
- Reach: root (a screen added between a user's select and paste).
- Verified: read.
- Port's handling: `sys/dev/wscons/wsdisplay.rs:141-143` zeroes it.
- Severity: wrong result (heap bytes into a tty).
- Fix: `M_ZERO`, or clear `SC_PASTE_AVAIL`.

### EXT-158: the eleventh CSI argument touches `args[10]`

- Where: `sys/dev/wscons/wsemul_vt100.c:1087` (and 1105, 879, 886).
- What: after ten arguments the terminators still do `if (edp->args[edp->nargs] < 0)
  edp->args[edp->nargs] = VT100_EMUL_ARG_CLAMP;` (1087-1088) on `int args[10]`, which lands
  on `modif1`/`modif2` and padding.
- Reach: user (`printf '\033[1;2;3;4;5;6;7;8;9;10;11m'` on a wscons tty).
- Verified: read.
- Port's handling: `sys/dev/wscons/wsemul_vt100.rs:87-89` skips the clamp.
- Severity: cosmetic (inside the structure; at worst the sequence is misread).
- Fix: clamp only `if (edp->nargs < VT100_EMUL_NARGS)`.

### EXT-159: an underlined space is written two lines up, before the frame buffer

- Where: `sys/dev/rasops/rasops8.c:370`, `rasops8_putchar16`.
- What: `if (uc == ' ') { while (height--) rp[0] = rp[1] = rp[2] = rp[3] = stamp[0]; }`
  (370-372) never advances `rp`, unlike the 8 and 12 pixel versions. The underline then
  does `DELTA(rp, -(ri->ri_stride << 1), int32_t *);` (391), two lines above the cell, which
  is before `ri_bits` on row 0 without a top margin.
- Reach: user (an underlined space on an 8-bpp display with a 16-pixel font).
- Verified: read.
- Port's handling: same fault (`sys/dev/rasops/rasops8.rs:66-70`).
- Severity: memory corruption (on row 0; otherwise a wrong display).
- Fix: advance `rp` by `ri_stride` in the space loop.

### EXT-160: rasops8 reads 4 bytes per glyph row

- Where: `sys/dev/rasops/rasops8.c:136`, `rasops8_putchar`.
- What: `fb = fr[3] | (fr[2] << 8) | (fr[1] << 16) | (fr[0] << 24);` (136) reads up to 3
  bytes past the last glyph of a narrow font; the bits are discarded.
- Reach: user.
- Verified: read.
- Port's handling: `sys/dev/rasops/rasops8.rs:75-78` reads `stride` bytes.
- Severity: cosmetic.
- Fix: read only the row's bytes.

### EXT-161: audio(4) saves and restores uninitialised mixer entries

- Where: `sys/dev/audio.c:1282`, `audio_attach`/`audio_activate`.
- What: `mix_ents` is allocated without `M_ZERO` (1282-1283), and class entries keep garbage
  `dev` values passed to `get_port`/`set_port` on suspend and resume (1339-1351).
- Reach: root (suspend and resume; the drivers checked bound `dev`, so no effect found).
- Verified: read.
- Port's handling: `sys/dev/audio.rs:77-78` zeroes it.
- Severity: cosmetic.
- Fix: `M_ZERO`, or skip class entries.

### EXT-162: `auich_calibrate` divides by a zero interval

- Where: `sys/dev/pci/auich.c:1469`, `auich_calibrate`.
- What: `actual_48k_rate = (bytes * 250000) / wait_us;` (1469), where the loop can end on the
  first poll in the same microsecond (1410-1422).
- Reach: firmware/hardware (a controller showing completion at once; the timing is not
  demonstrated).
- Verified: read.
- Port's handling: `sys/dev/pci/auich.rs:89-90` divides by at least 1.
- Severity: crash.
- Fix: treat 0 as a failed calibration.

### EXT-163: deleting an azalia codec leaks its connection lists

- Where: `sys/dev/pci/azalia.c:2647`, `azalia_codec_delete`.
- What: `this->w` is freed (2647-2651) but no widget's `connections` (3409).
- Reach: firmware/hardware (every attach that drops a codec, e.g. HDMI next to analog).
- Verified: read.
- Port's handling: `sys/dev/pci/azalia.rs:83` frees them.
- Severity: leak.
- Fix: free each widget's `connections` first.

### EXT-164: azalia trusts the codec's Connection Select index

- Where: `sys/dev/pci/azalia.c:2344` (and 2398; `azalia_codec.c:1778`).
- What: `w->selected < sizeof(w->connections)` (2344-2346) compares with a pointer's size,
  not `nconnections`, then reads `w->connections[w->selected]`; the mixer reads with the raw
  index.
- Reach: firmware/hardware (a codec index past its list).
- Verified: read.
- Port's handling: `sys/dev/pci/azalia.rs:105-107`, `azalia_codec.rs:74-75`.
- Severity: wrong result (an out-of-bounds heap read).
- Fix: `w->selected < w->nconnections`.

### EXT-165: a failed mixer growth is ignored and the array overrun

- Where: `sys/dev/pci/azalia_codec.c:1276` (and 1329), `azalia_mixer_init`.
- What: `err = azalia_mixer_ensure_capacity(this, this->nmixers + 3);` (1276-1277) is not
  checked, and three `mixers[this->nmixers++]` entries follow.
- Reach: alloc-failure.
- Verified: read.
- Port's handling: `sys/dev/pci/azalia_codec.rs:70-71` returns the error.
- Severity: memory corruption.
- Fix: `if (err) return err;`.

### EXT-166: azalia writes an uninitialised digital-control value

- Where: `sys/dev/pci/azalia.c:2926` (and `azalia_codec.c:2223-2227`).
- What: `CORB_GET_DIGITAL_CONTROL`'s error is ignored, and `v = (v & ~CORB_DCC_DIGEN) & 0xff;`
  (2926) sends stack garbage when it timed out.
- Reach: firmware/hardware (a codec not answering).
- Verified: read.
- Port's handling: `sys/dev/pci/azalia.rs:93-96`, `azalia_codec.rs:76-77` use 0.
- Severity: wrong result.
- Fix: skip the SET when the GET fails.

### EXT-167: GPIOATTACH prints through the wrong print function

- Where: `sys/dev/gpio/gpio.c:419`, `gpioioctl`.
- What: `config_found_sm(..., &ga, gpiobus_print, ...)` (419) passes a `struct
  gpio_attach_args` that `gpiobus_print` reads as `gpiobus_attach_args`, printing the softc
  as a name.
- Reach: root (`gpioctl ... attach` with an unknown driver name).
- Verified: read.
- Port's handling: `sys/dev/gpio/gpio.rs:55-57`.
- Severity: cosmetic.
- Fix: use a print function for `gpio_attach_args`.

### EXT-168: more than two IOMMU cells overrun `cells[2]`

- Where: `sys/dev/ofw/ofw_misc.c:1103`, `iommu_device_lookup_idx`.
- What: `KASSERT(icells <= 2);` (1098), then `cells[i] = cell[1 + i];` (1102-1103) into the
  callers' `uint32_t cells[2]`. Without `DIAGNOSTIC` (arm64 RAMDISK) the stack array
  overflows.
- Reach: firmware/hardware (a device tree with `#iommu-cells` of 3 or more).
- Verified: read.
- Port's handling: `sys/dev/ofw/ofw_misc.rs:84-85` copies two at most.
- Severity: memory corruption (a panic with DIAGNOSTIC).
- Fix: fail instead of asserting.

### EXT-169: IPMI 6-bit sensor names are read past the SDR

- Where: `sys/dev/ipmi.c:1139`, `ipmi_sensor_name`.
- What: the check `slen * 6 / 8 > bitslen` (1137) lets `slen` reach `bitslen * 4 / 3`, and
  `getbits` (1139) reads that far.
- Reach: firmware/hardware (a BMC's SDR).
- Verified: read.
- Port's handling: `sys/dev/ipmi.rs:113-115` reads 0 past the record.
- Severity: wrong result.
- Fix: check `slen > bitslen` as for 8-bit names.

### EXT-170: IPMI BT and SMIC receive trust the BMC's length

- Where: `sys/dev/ipmi.c:557` (and 387, 931), `smic_recvmsg`, `bt_recvmsg`,
  `ipmi_recvcmd`.
- What: `sts = smic_read_data(sc, &sc->sc_buf[idx++]);` (557) loops without a bound;
  `c_rxlen = len - 1` (387) comes from the BMC; `memcpy(c->c_data, ..., c->c_rxlen);` (931)
  ignores `c_maxrxlen` into callers' small stack buffers.
- Reach: firmware/hardware (a BT or SMIC BMC).
- Verified: read.
- Port's handling: `sys/dev/ipmi.rs:110-113` bounds them.
- Severity: memory corruption.
- Fix: bound the loop by `sc_buf`, and clamp `c_rxlen` to `c_maxrxlen`.

### EXT-171: an unknown IPMI interface type dereferences NULL

- Where: `sys/dev/ipmi.c:1576`, `ipmi_attach_common`.
- What: `ipmi_map_regs(sc, ia);` (1562) ignores its failure, and `sc->sc_if->name` (1576) is
  NULL for an unknown type.
- Reach: firmware/hardware (an ACPI `_IFT` outside 1..3 with a register resource).
- Verified: read.
- Port's handling: `sys/dev/ipmi.rs:123-124` stops.
- Severity: crash.
- Fix: check `ipmi_map_regs`.

### EXT-172: the IORT walks are not bounded by the table

- Where: `sys/arch/arm64/dev/acpiiort.c:82`, `acpiiort_attach` (and 156-186).
- What: `offset += node->length;` (82), `mapping_offset` and `map[i]` (164-172) are never
  checked against `hdr.length`.
- Reach: firmware/hardware (a malformed IORT).
- Verified: read.
- Port's handling: `sys/arch/arm64/dev/acpiiort.rs:66-67`.
- Severity: crash.
- Fix: check each offset against the table length.

### EXT-173: ddb's arm64 `$x30` is past the trapframe

- Where: `sys/arch/arm64/arm64/db_interface.c:90`, `db_regs[]`.
- What: `{ "x30", (long *)&DDB_REGS->tf_x[30], FCN_NULL, },` (90), but `tf_x` has 30
  entries (`arch/arm64/include/frame.h:35`); x30 is `tf_lr`.
- Reach: root (ddb: `show registers` prints a wrong x30, and writing it corrupts the next
  global).
- Verified: read.
- Port's handling: `sys/arch/arm64/arm64/db_interface.rs:75` uses `tf_lr`.
- Severity: wrong result.
- Fix: `&DDB_REGS->tf_lr`.

### EXT-174: pciecam passes a NULL extent to `extent_free`

- Where: `sys/dev/fdt/pciecam.c:223`, `pciecam_attach`.
- What: `extent_create(..., EX_NOWAIT | EX_FILLED)` (211, 216) is unchecked, and
  `extent_free(sc->sc_ioex, ...)` (223, 226) dereferences it.
- Reach: alloc-failure.
- Verified: read.
- Port's handling: `sys/dev/fdt/pciecam.rs:53-54`.
- Severity: crash.
- Fix: check the results.

### EXT-181: pckbd_enable's polled ACK is taken by the interrupt handler

- Where: `sys/dev/pckbc/pckbd.c:499-507`, `pckbd_enable`, with `sys/dev/ic/pckbc.c:1041`,
  `pckbcintr`.
- What: opening `/dev/wskbd0` enables the keyboard with `pckbc_poll_cmd(KBC_ENABLE)`
  while IRQ1 is live. The keyboard answers the 0xF4 with its ACK at once and raises IRQ1;
  when the opener runs on the CPU that takes IRQ1, `pckbcintr` reads the ACK first (the
  slot is not polling) and hands 0xFA to `pckbd_input`; the poll times out:
  `pckbd_enable: command error` and the open fails with EIO.
- Seen: OpenBSD 8.0 on QEMU q35 `-smp 2` (`diff-openbsd probe`, 2026-10-09): 8 opens of
  `/dev/wskbd0` with a key typed during each: 48, 48, 0, 0, 0, 0, 48, 48 bytes, every
  failure printed `pckbd_enable: command error`. EmiBSD the same (4 of 6 `smoke-pckbc` runs
  before the retry; debug prints showed every failure with the opener on cpu0).
- Reach: root (`/dev/wskbd0` is 0600; X servers open it).
- Port's handling: the C is kept (`sys/dev/pckbc/pckbd.rs`); `smoke-pckbc` retries the open.
- Fix: send KBC_ENABLE through the queue (`pckbc_enqueue_cmd` with `sync`) outside
  autoconfiguration, or keep the slot polling while the command is in flight.

### EXT-182: spkr's note length divides by zero

- Where: `sys/dev/isa/spkr.c:183-199`, `playtone`.
- What: each sustain dot doubles `sdenom`; `value * sdenom` (an `int`, `value` up to 64)
  overflows to 0 after 26 to 30 dots, and `whole * snum / (value * sdenom)` divides by
  zero. A play string written to `/dev/speaker` reaches it.
- Seen: OpenBSD 8.0 on QEMU q35 (`diff-openbsd probe --pcspk`, 2026-10-09): `echo
  "c.............................." > /dev/speaker` (30 dots) as root: `kernel: integer
  divide fault trap, code=0`, `Stopped at      playtone+0xa4:  idivl   %ecx,%eax`, `ddb{0}>`.
- Reach: root (`/dev/speaker` is 0600).
- Port's handling: `sys/dev/isa/spkr.rs` multiplies with wrapping and the division panics,
  as the C traps (same fault).
- Fix: bound the dots (`sustain`) or the divisor before dividing.

### EXT-183: pckbd's untranslated set-2 extended table is one entry short

- Where: `sys/dev/pckbc/pckbd.c:684-700`, `pckbd_xtbl2_ext`.
- What: the `/* 0x00 */` row has 15 entries instead of 16, so the table holds 127 and
  every later key sits one index below its row comment (right Alt at 0x10, Ctrl-Break 0xc6
  at 0x7d).
- Reach: firmware/hardware (only an 8042 that does not translate to set 1 uses it; QEMU's
  translates).
- Port's handling: kept (`sys/dev/pckbc/pckbd.rs`).
- Fix: a 16th `0` in the first row.

### EXT-184: pms's Elantech v1 parity table flips on each enable

- Where: `sys/dev/pckbc/pms.c:2088-2089`, `pms_enable_elantech_v1`.
- What: `parity[i] = parity[i & (i - 1)] ^ 1` starting at `i = 0` toggles `parity[0]`
  each time, so every enable after the first inverts the whole table and the v1 packet
  parity checks then fail (Linux sets `parity[0] = 1` and starts at 1).
- Reach: device (an Elantech v1 touchpad, re-enabled on resume or reopen).
- Port's handling: kept (`sys/dev/pckbc/pms.rs`).
- Fix: start the loop at 1 with `parity[0] = 1`.

### EXT-185: pms's Elantech read-back loops break on failure

- Where: `sys/dev/pckbc/pms.c:1726`, `1769` and `1801`, `elantech_set_absolute_mode_v1`
  to `_v3`.
- What: `if (pms_spec_cmd(..) || pms_spec_cmd(..) || pms_get_status(..) == 0) break;`
  leaves the retry loop when a command fails as when the status read succeeds; v1 then
  checks `resp[0]`, which the failed path never set.
- Reach: device.
- Port's handling: kept; `sys/dev/pckbc/pms.rs` zeroes the buffer, so v1 reports the
  failure where the C reads stack garbage.
- Fix: retry while a command fails, break only on a good status.

### EXT-186: eap refuses 8-bit unsigned audio

- Where: `sys/dev/pci/eap.c:850-855`, `eap_set_params`.
- What: `case AUDIO_ENCODING_ULINEAR_LE: case AUDIO_ENCODING_ULINEAR_BE: if (p->precision
  != 8) return EINVAL;` has no `break` and falls into `default: return (EINVAL);`, so
  8-bit unsigned, which the hardware plays, is always refused.
- Reach: user (an audio(4) client in `_sndiop`); audio(4) then falls back to 16-bit.
- Port's handling: `sys/dev/pci/eap.rs` returns EINVAL for both (same result).
- Fix: `break;` after the precision check.

### EXT-187: viomb's deflate count is one short

- Where: `sys/dev/pv/viomb.c:345-350`, `viomb_deflate`.
- What: when the balloon list runs out after `i` pages were moved to the request,
  `b->bl_nentries = i - 1` records one fewer; the request still sends `nvpages` words.
- Reach: firmware/hardware (the host asks to deflate more than was inflated).
- Port's handling: kept (`sys/dev/pv/viomb.rs`).
- Fix: `b->bl_nentries = i;` and send `i` words.

### EXT-188: viomb queues its page arrays with the wrong direction

- Where: `sys/dev/pv/viomb.c:319-320` and `369-370`, with the file's `VRING_READ 0`
  (58) and `sys/dev/pv/virtio.c:724-725`.
- What: the inflate and deflate arrays, which the device reads, go to
  `virtio_enqueue_p(..., VRING_READ)`, and `write == 0` sets `VRING_DESC_F_WRITE`
  (device-writable). The virtio specification has the driver's data read-only to the
  device; QEMU does not check.
- Reach: firmware/hardware (a strict device would refuse the requests).
- Port's handling: kept (`sys/dev/pv/viomb.rs`).
- Fix: pass 1 (driver-written) for these buffers.

### EXT-189: viogpu's attach unwinds with the wrong address

- Where: `sys/dev/pv/viogpu.c:296` and `303`, `viogpu_attach`.
- What: `bus_dmamem_unmap(vsc->sc_dmat, (caddr_t)&sc->sc_fb_dma_kva, ...)` and
  `(caddr_t)&sc->sc_cmd` pass the address of the softc member, not the mapped kva.
- Reach: alloc-failure (a later step of the attach failing).
- Port's handling: `sys/dev/pv/viogpu.rs` unmaps the kva.
- Fix: drop the `&`.

### EXT-190: viogpu's soft interrupt is never used

- Where: `sys/dev/pv/viogpu.c:173`.
- What: `softintr_establish(IPL_TTY, viogpu_rx_soft, vsc)`'s handle is thrown away, so
  `viogpu_rx_soft` (349) can never be scheduled, and the handler is leaked.
- Reach: firmware/hardware (every attach).
- Port's handling: kept (`sys/dev/pv/viogpu.rs`).
- Fix: keep the handle and schedule it from the queue's done routine, or remove both.

### EXT-191: viogpu's attach line runs into virtio's

- Where: `sys/dev/pv/viogpu.c:149-214`, `viogpu_attach`, and `275`.
- What: nothing ends `viogpu0 at virtioN` before `virtio_attach_finish` prints
  `virtioN: msix per-VQ`, and the size comes on a line of its own.
- Seen: OpenBSD 8.0 on QEMU virt with `virtio-gpu-pci` (`diff-openbsd probe`):
  `viogpu0 at virtio32virtio32: msix per-VQ`, then `: 1280x800, 32bpp`.
- Reach: firmware/hardware.
- Port's handling: the same lines (`smoke-viogpu` expects them).
- Fix: `printf("\n")` before `virtio_attach_finish`, the size after the name.

## boot loaders

### EXT-175: a long TFTP path overflows the request buffer

- Where: `sys/lib/libsa/tftp.c:169`, `tftp_makereq`.
- What: `wbuf` has `FNAME_SIZE + 6` bytes of space (154-158); `bcopy(h->path, wtail, l +
  1); ... bcopy("octet", wtail, 6);` (168-171) writes `strlen + 7` bytes unchecked.
- Reach: root (`boot tftp:<path>` with a path over 131 bytes, typed or in boot.conf).
- Verified: read.
- Port's handling: `sys/lib/libsa/tftp.rs:130` answers ENOENT.
- Severity: memory corruption (the loader's stack).
- Fix: refuse a path longer than `FNAME_SIZE`.

### EXT-176: libsa's `cd9660_open` walks records past the block

- Where: `sys/lib/libsa/cd9660.c:240`, `cd9660_open`.
- What: a new block is read only at a block boundary (216); otherwise `dirmatch(path, dp)`
  (240) and `dp += length` (242) trust the record's length.
- Reach: device (a crafted ISO; amd64 cdboot and efiboot, arm64 efiboot).
- Verified: read.
- Port's handling: `sys/lib/libsa/cd9660.rs:100` ends the table there.
- Severity: wrong result (an out-of-bounds read).
- Fix: stop when a record crosses the block.

### EXT-177: arm64 efiboot's heap has no limit

- Where: `sys/lib/libsa/alloc.c:185`, `alloc`.
- What: `top += ALIGN(sizeof(unsigned)) + ALIGN(size);` (185) is checked only `#ifdef
  HEAP_LIMIT` (186-189), which arm64 efiboot does not define (amd64 does); its heap is 1 MB.
- Reach: device (a crafted ISO whose path table size makes `cd9660_open` allocate and read
  past the heap).
- Verified: read.
- Port's handling: `sys/arch/arm64/stand/efiboot/heap.rs:47` panics "heap full".
- Severity: memory corruption.
- Fix: define `HEAP_LIMIT` as heap + heapsiz.

### EXT-178: `efistrategy` copies whole sectors past the caller's size

- Where: `sys/arch/arm64/stand/efiboot/efidev.c:106` (and amd64 `efidev.c:115`), `efid_io`.
- What: `nsect = (size + DEV_BSIZE - 1) / DEV_BSIZE;` (532), then `memcpy(buf, ...,
  DEV_BSIZE * nsect);` (106-107) writes up to 511 bytes past a buffer of `size` bytes.
- Reach: device. A crafted FFS whose `fs_bsize` is not a multiple of 512 (libsa `ufs.c`
  reads `fs_bsize` bytes into `alloc(fs_bsize)`, 123-127).
- Verified: read.
- Port's handling: `sys/arch/arm64/stand/efiboot/efidev.rs:87`,
  `sys/arch/amd64/stand/efiboot/efidev.rs:89` use a bounce buffer.
- Severity: memory corruption.
- Fix: copy only `size` bytes of the last sector.

### EXT-179: arm64 efiboot's FDT walkers read past the blob

- Where: `sys/arch/arm64/stand/efiboot/fdt.c:169`, `skip_property` (and the other walkers).
- What: `ptr += 3 + roundup(size, 4) / 4;` (174) with a size from the blob, never checked
  against `fh_size`; `fdt_get_str` accepts `num == strings_size` (134).
- Reach: firmware/hardware (a malformed DTB from firmware or `/etc/firmware/dtb/`).
- Verified: read.
- Port's handling: `sys/arch/arm64/stand/efiboot/fdt.rs:68` bounds every read.
- Severity: wrong result (out-of-bounds reads).
- Fix: check each step against the tree's end.

### EXT-180: libsa's `readudp` checksums `uh_ulen` bytes past the datagram

- Where: `sys/lib/libsa/netudp.c:220`, `readudp`.
- What: `n = ntohs(uh->uh_ulen) + sizeof(*ip);` (220) is checked only against 1522, then
  `in_cksum(ui, n)` (231) reads past the caller's 554-byte buffer.
- Reach: remote (a datagram to a netbooting loader; the bad checksum then drops it).
- Verified: read.
- Port's handling: `sys/lib/libsa/netudp.rs:73` drops it.
- Severity: cosmetic (an out-of-bounds read whose result is discarded).
- Fix: drop when `uh_ulen` exceeds what was received.
