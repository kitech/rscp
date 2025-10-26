import dl
import vcp
import time
import os

#flag -laddr2linec -L/opt/cargocc/debug/ -Wl,-rpath=/opt/cargocc/debug

// not works with tcc!!!
fn main() {
    name := '_ZN10addr2linec10new_loader17h3499fb07c074be08E'
    name2 := '_ZN10addr2linec13find_location17he5cfe889e8a6c57bE'
    name3 := '_ZN10addr2linec11drop_loader17hd4fe70f7e0ebfb03E'
    
    adr := dl.sym(vnil, name)
    assert adr != vnil, name
    
    fno := funcof(adr, fn(_ voidptr) voidptr{return vnil})
    filename := os.executable()
    btime := time.now()
    loader := fno( filename.str)
    vcp.info(loader, (time.now()-btime).str())

    adr2 := dl.sym(vnil, name2)
    assert adr2 != vnil, name2

    buf := malloc(123)   
    btime = time.now()
    adr5 := u64(main)    
    fno2 := funcof(adr2,  fn(_ voidptr, _ u64, _ voidptr) voidptr {return vnil})
    loader = fno2(loader, adr5, buf)
    vcp.info(tos3(charptr(buf)), (time.now()-btime).str())
    loader = fno2(loader, adr5, buf)
    vcp.info(tos3(charptr(buf)), (time.now()-btime).str())
   
    adr3 := dl.sym(vnil, name3)
    assert adr3 != vnil, name3
    
    btime = time.now()
    fno3 := funcof(adr3,  fn(_ voidptr) {})
    fno3(loader)
    vcp.info(tos3(charptr(buf)), (time.now()-btime).str())    
}
