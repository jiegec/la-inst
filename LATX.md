# New instructions in LATX

See [issue #2](https://github.com/jiegec/la-inst/issues/2).

## AES

- aes.fr.{dec,enc}
- aes.kg
- aes.lr.{dec,enc}
- aes.mc.{dec,enc}
- aes.sb.{dec,enc}
- aes.sr.{dec,enc}
- aes{128,192,256}.{dec,enc}

## Hash

- md5.{4r,ms}
- sha1.{hash.4r,ms.1,ms.2}
- sha256.{hash.2r,ms.1,ms.2}
- sha512.{hash.r.1,hash.r.2,ms.1,ms.2}

## Integer computation

### Widening

- v{add,sub}w.{d.d.w,d.d.wu,h.h.b,h.h.bu,w.w.h,w.w.hu}: add x-bit element and 2x-bit element, get 2x-bit sum
- v{add,sub}w{h,l}.{d.w,d.wu,d.wu.w,h.b,h.bu,h.bu.b,q.d,q.du,q.du.d,w.h,w.hu,w.hu.h}: add x-bit element and x-bit element, get 2x-bit sum
- vmaddw{h,l}.{d.w,d.wu,d.wu.w,h.b,h.bu,h.bu.b,q.d,q.du,q.du.d,w.h,w.hu,w.hu.h}: multiply x-bit element and x-bit element, add to 2x-bit sum
- vmulw{hl}.{d.w,d.wu,d.wu.w,h.b,h.bu,h.bu.b,q.d,q.du,q.du.d,w.h,w.hu,w.hu.h}: multiply x-bit element and x-bit element, get 2x-bit product
- vmulxw.{d.w,d.wu,h.b,h.bu,w.h,w.hu}
- vaccsadw.{d.w,d.wu,h.b,h.bu,w.h,w.hu}
- vsadw.{d.w,d.wu,h.b,h.bu,w.h,w.hu}
- vs{add,sub}w.{d.d.w,du.du.wu,h.h.b,hu.hu.bu,w.w.h,wu.wu.hu}: saturated add/subtract x-bit element and 2x-bit element, get 2x-bit sum
- vsadda.{b,d,h,w}: saturated add absolute value
- vssub.{b.bu.bu,bu.b.bu,bu.bu.b,d.du.du,du.d.du,du.du.d,h.hu.hu,hu.h.hu,hu.hu.h,w.wu.wu,wu.w.wu,wu.wu.w}: saturated subtract with different signedness
- v{sra,srl}rneni.{b.h,d.q,h.w,w.d}
- vs{sra,srl}rneni.{b.h,bu.h,d.q,du.q,h.w,hu.w,w.d,wu.d}

### Misc

- vextl.{d.b,d.bu,d.h,d.hu,w.b,w.bu}
- vhalfd.{b,bu,d,du,h,hu,w,wu}
- vhminpos.{d.hu,q.hu,w.hu}
- v{min,max}a.{b,d,h,w}: minimum/maxmium absolute value
- vmepatmsk.v: make pattern mask
- vmsk{copy,fill}.b
- vmuh.{bu.b,du.d,hu.h,wu.w}
- vrandsign{,i}.{b,h}
- vrorsign{,i}.{b,h}
- vseli.{d,h,w}
- vshuf2.d
- vshuf4.w
- vshufi{1,2}.{b,h}
- vshufi3.b
- vshufi4.b
- vsignsel.{d,w}

### Dot product

- vdp2{,add,sub}.{d.w,d.wu.w,du.wu,h.b,h.bu.b,hu.bu,q.d,q.du.d,qu.du,w.h,w.hu.h,wu.hu}
- vdp4{,add,sub}.{d.h,d.hu,d.hu.h,q.w,q.wu,q.wu.w,w.b,w.bu,w.bu.b}

### Horizontal add

- vhadd4.h.bu
- vhadd8.d.bu

### Polynomial

- vpdp2{,add}.q.d
- vpmaddw{h,l}.{d.w,h.b,q.d,w.h}
- vpmulw{h,l}.{d.w,h.b,q.d,w.h}
- vpmu{h,l}{,acc}.{d,w}

## Extraction

- vextr.v
- vextrcol{,i}.{b,d,h,w}

## Bitwise operation

- vbitmvnzi.b
- vbitmvzi.b
- vbstrc{12,12i,21,21i}.{b,d,h,w}
- vclrstr{i,r,v}.v
- vclrtail.{b,h}

## Floating point computation

### Add + sub

- vfaddsub.{d,s}
- vfsubadd.{d,s}
- vfmaddsub.{d,s}
- vfmsubadd.{d,s}

### Misc

- vfrstm.{b,h}
- vfscaleb.{b,h}

### Complex dot product

- vcdp2{,add}.im.q.w
- vcdp2{,add}.re.q.w
- vcdp4{,add}.im.d.h
- vcdp4{,add}.re.d.h
