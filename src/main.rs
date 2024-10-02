use std::fs;
//use std::collections::HashMap;
use std::collections::BTreeMap;

fn main() {
    let mut args = std::env::args();
    println!("{:?}",args);
    let data: Vec<u8> =
        fs::read(args.nth(1)
            .expect("no name given"))
            .expect("can't open file");

    let mut addr = 0x819_D420;
    addr -= 0x800_0000;
    let sets = read_u32(&data, addr, 5);
    println!("{:08X?}",sets);


    let mut entries = Vec::new();

    entries.push(read_entries(&data, sets[0] as usize, 88));
    entries.push(read_entries(&data, sets[1] as usize, 43));
    entries.push(read_entries(&data, sets[2] as usize, 11));
    entries.push(read_entries(&data, sets[3] as usize, 21));
    entries.push(read_entries(&data, sets[4] as usize, 3));

    let pals = entries.iter().flatten().map(|x| x[0]).filter(|&x| x!=0).collect::<Vec<u32>>();
    println!("{:8x?}",pals);

    let mut pals_map = BTreeMap::new();
    let mut c = 1;
    pals.iter().for_each(|x| {
        if !pals_map.contains_key(x) {
            pals_map.insert(x,c);
            c += 1;
        }
    });
    println!("{:8x?}",pals_map.iter());
    println!("{}",pals_map.len());

    /*
    for (paddr, pnum) in pals_map.iter() {
        create_palette(&data,**paddr as usize,*pnum);
    }
    */

    let tiles = entries.iter().flatten().map(|x| x[3])
        .filter(|&x| x!=0)
        .collect::<Vec<u32>>();
    println!("{:8x?}",tiles);

    let mut tiles_map = BTreeMap::new();
    let mut c = 1;
    tiles.iter().for_each(|x| {
        if !tiles_map.contains_key(x) {
            tiles_map.insert(x,c);
            c += 1;
        }
    });
    println!("{:8x?}",tiles_map.iter());
    println!("{}",tiles_map.len());

    /*
    for e in tiles_map.iter() {
        let (a,uncomp) = lz77_uncomp(&data,(*e.0-0x800_0000) as usize);
        println!("a:{:8x} c:{:8} u:{:8} r:{:5}",*e.0,uncomp,a.len(),uncomp*10000/a.len());
        println!("{:02x?}",a[0..64].to_vec());
    }
    */

    let supertiles = entries.iter().flatten().map(|x| x[4])
        .filter(|&x| x!=0)
        .collect::<Vec<u32>>();
    println!("{:8x?}",supertiles);

    let mut supertiles_map = BTreeMap::new();
    let mut c = 1;
    supertiles.iter().for_each(|x| {
        if !supertiles_map.contains_key(x) {
            supertiles_map.insert(x,c);
            c += 1;
        }
    });
    println!("{:8x?}",supertiles_map.iter());
    println!("{}",supertiles_map.len());



    let supermaps = entries.iter().flatten().map(|x| x[5])
        .filter(|&x| x!=0)
        .collect::<Vec<u32>>();
    println!("{:8x?}",supermaps);

    let mut supermaps_map = BTreeMap::new();
    let mut c = 1;
    supermaps.iter().for_each(|x| {
        if !supermaps_map.contains_key(x) {
            supermaps_map.insert(x,c);
            c += 1;
        }
    });
    println!("{:8x?}",supermaps_map.iter());
    println!("{}",supermaps_map.len());



    let dims = entries.iter().flatten().map(|x| x[6])
        .filter(|&x| x!=0)
        .collect::<Vec<u32>>();
    println!("{:8x?}",dims);

    let mut dims_map = BTreeMap::new();
    let mut c = 1;
    dims.iter().for_each(|x| {
        if !dims_map.contains_key(x) {
            dims_map.insert(x,c);
            c += 1;
        }
    });
    println!("{:8x?}",dims_map.iter());
    println!("{}",dims_map.len());

    let mut entries2 = entries.iter().flatten()
        .map(|x| (x[0],x[3],x[4],x[5],x[6]))
        .filter(|x| !(x.0==0 || x.1==0 || x.2==0 || x.3==0 || x.4==0))
        //.map(|x| (pals_map[&x.0],tiles_map[&x.1],supertiles_map[&x.2],supermaps_map[&x.3]))
        .collect::<Vec<_>>();

    entries2.sort();
    entries2.dedup();

    println!("{:8x?}",entries2);
    println!("{}",entries2.len());

    /*
    let paddr = entries2[0].0;
    let pdata: Vec<u16> = [
        vec![0; 0x40],
        read_u16(&data, (paddr-0x800_0000) as usize, 0xC0)
        ].concat();

    let taddr = entries2[0].1;
    let (tdata,_) = lz77_uncomp(&data, (taddr-0x800_0000) as usize);

    let staddr = entries2[0].2;
    let (stdata,_) = lz77_uncomp(&data, (staddr-0x800_0000) as usize);
    let stdata = read_u16(&stdata, 0, stdata.len()/2);
    */

    /*
    let raster: Vec<u8> = render_tile(&tdata,stdata[4]).into_iter().flatten().collect();
    let bmp = gba_to_bmp(&pdata, &raster, 8, 8);
    let _ = fs::write("test4.bmp", bmp);
    */

    /*
    let stdata: Vec<Vec<u16>> = stdata[..].chunks_exact(5)
        .map(|e| vec![e[1],e[2],e[3],e[4]])
        .collect();
    */

    /*
    let width = 256;
    let height = (stdata.len()+15)/16*16;

    let mut raster = vec![vec![0; width]; height];

    for (i,e) in stdata.iter().enumerate() {
        for (j,f) in e.iter().enumerate() {
            let y = i/16*16 + j/2*8;
            let x = i%16*16 + j%2*8;
            draw(&mut raster, &render_tile(&tdata, *f), x, y);
        }
    }

    let bmp = gba_to_bmp(&pdata, &raster.into_iter().flatten().collect(), width as i32, height as i32);
    let _ = fs::write("test5.bmp", bmp);
    */

    /*
    let smaddr = entries2[0].3;
    let (smdata,_) = lz77_uncomp(&data, (smaddr-0x800_0000) as usize);
    let smdata = read_u16(&smdata, 0, smdata.len()/2);
    //println!("{:4?}",smdata);
    //println!("{} {} {}",smdata.len(),smdata.iter().max().unwrap(),stdata.len());

    let map_width_tiles = 120;
    let map_height_tiles = 80;
    let map_width = map_width_tiles*2;
    let map_height = map_height_tiles*2;
    let mut map_data = vec![vec![0 as u16; map_width]; map_height];

    for (i,e) in smdata.iter().enumerate() {
        let y = i/map_width_tiles*2;
        let x = i%map_width_tiles*2;
        map_data[y  ][x  ] = stdata[*e as usize][0];
        map_data[y  ][x+1] = stdata[*e as usize][1];
        map_data[y+1][x  ] = stdata[*e as usize][2];
        map_data[y+1][x+1] = stdata[*e as usize][3];
    }

    let width = map_width*8;
    let height = map_height*8;

    let mut raster = vec![vec![0; width]; height];

    for (i,e) in map_data.iter().enumerate() {
        for (j,f) in e.iter().enumerate() {
            let y = i*8;
            let x = j*8;
            draw(&mut raster, &render_tile(&tdata, *f), x, y);
        }
    }

    let bmp = gba_to_bmp(&pdata, &raster.into_iter().flatten().collect(), width as i32, height as i32);
    let _ = fs::write("test6.bmp", bmp);
    */

    for e in entries2.iter() {
        let bmp = create_background_bmp(&data,
            e.0 as usize, e.1 as usize, e.2 as usize,
            e.3 as usize, e.4 as usize);
        let _ = fs::write(format!("bg_{:02}.bmp",supermaps_map[&e.3]), bmp);
    }
}

fn create_background_bmp (data: &Vec<u8>,
    pal_addr: usize, tile_addr: usize,
    stile_addr: usize, smap_addr: usize,
    dim_addr: usize) -> Vec<u8> {
    let pal_data: Vec<u16> = [
        vec![0; 0x40],
        read_u16(&data, pal_addr-0x800_0000, 0xC0)
        ].concat();

    let (tile_data,_) = lz77_uncomp(&data, tile_addr-0x800_0000);

    let (stile_data,_) = lz77_uncomp(&data, stile_addr-0x800_0000);
    let stile_data = read_u16(&stile_data, 0, stile_data.len()/2);
    let stile_data: Vec<Vec<u16>> = stile_data[..].chunks_exact(5)
        .map(|e| vec![e[1],e[2],e[3],e[4]])
        .collect();

    let (smap_data,_) = lz77_uncomp(&data, smap_addr-0x800_0000);
    let smap_data = read_u16(&smap_data, 0, smap_data.len()/2);

    //let map_width_tiles = 60;
    //let map_height_tiles = 80;
    let dim = read_u16(&data, dim_addr-0x800_0000, 2);
    let map_width_tiles = dim[0] as usize;
    let map_height_tiles = dim[1] as usize;
    let map_width = map_width_tiles*2;
    let map_height = map_height_tiles*2;
    let mut map_data = vec![vec![0 as u16; map_width]; map_height];

    for (i,e) in smap_data.iter().enumerate() {
        let y = i/map_width_tiles*2;
        let x = i%map_width_tiles*2;
        map_data[y  ][x  ] = stile_data[*e as usize][0];
        map_data[y  ][x+1] = stile_data[*e as usize][1];
        map_data[y+1][x  ] = stile_data[*e as usize][2];
        map_data[y+1][x+1] = stile_data[*e as usize][3];
    }

    let width = map_width*8;
    let height = map_height*8;

    let mut raster = vec![vec![0; width]; height];

    for (i,e) in map_data.iter().enumerate() {
        for (j,f) in e.iter().enumerate() {
            let y = i*8;
            let x = j*8;
            draw(&mut raster, &render_tile(&tile_data, *f), x, y);
        }
    }

    gba_to_bmp(&pal_data, &raster.into_iter().flatten().collect(),
        width as i32, height as i32)
}

fn draw (des: &mut Vec<Vec<u8>>, src: &Vec<Vec<u8>>, x: usize, y: usize) {
    for (j,row) in src.iter().enumerate() {
        for (i,pixel) in row.iter().enumerate() {
            des[y+j][x+i] = *pixel;
        }
    }
}

fn render_tile (tdata: &Vec<u8>, bg_text: u16) -> Vec<Vec<u8>> {
    //tile.iter().map(|x| vec![x&15, x>>4]).flatten().collect()
    let tile_num = bg_text & 1023;
    let hflip = (bg_text & 1024) != 0;
    let vflip = (bg_text & 2048) != 0;
    let pal_num = (bg_text>>12) as u8;

    let offset = (tile_num*32) as usize;

    let mut tile = tdata[offset..offset+32].to_vec().iter()
        .map(|e| vec![e&15, e>>4]).flatten()
        .map(|e| e+pal_num*16).collect::<Vec<u8>>()
        .chunks_exact(8).map(|e| e.to_vec())
        .collect::<Vec<Vec<u8>>>();

    if hflip {
        tile.iter_mut().for_each(|e| e.reverse());
    }

    if vflip {
        tile.reverse();
    }

    tile
}

fn read_u32 (data: &Vec<u8>, addr: usize, len: usize) -> Vec<u32> {
    data[addr..addr+len*4]
        .chunks_exact(4)
        .map(|e| u32::from_le_bytes(e.try_into().unwrap()))
        .collect()
}

fn read_u16 (data: &Vec<u8>, addr: usize, len: usize) -> Vec<u16> {
    data[addr..addr+len*2]
        .chunks_exact(2)
        .map(|e| u16::from_le_bytes(e.try_into().unwrap()))
        .collect()
}

fn read_u8 (data: &Vec<u8>, addr: usize, len: usize) -> Vec<u8> {
    data[addr..addr+len].to_vec()
}

fn gba_to_bmp (pal: &Vec<u16>, raster: &Vec<u8>, w: i32, h: i32) -> Vec<u8> {
    let size: u32 = 14+40+4*256+raster.len() as u32;
    let signature = [b'B',b'M'].to_vec();
    let offset: u32 = 14+40+4*256;
    let colors: u32 = 256;

    let palette = pal.iter()
        .map(|x| {
            let mut b: u8 = ((x>>10)&31).try_into().unwrap();
            let mut g: u8 = ((x>> 5)&31).try_into().unwrap();
            let mut r: u8 = ( x     &31).try_into().unwrap();
            b = b*8 + b/4;
            g = g*8 + g/4;
            r = r*8 + r/4;
            [b,g,r,0].to_vec()
        })
        .flatten()
        .collect::<Vec<u8>>();

    let bytes = [
        signature,
        size.to_le_bytes().to_vec(),
        [0,0,0,0].to_vec(), // reserved
        offset.to_le_bytes().to_vec(), // offset to raster data
        [40,0,0,0].to_vec(), // info header size
        w.to_le_bytes().to_vec(), // width
        (0-h).to_le_bytes().to_vec(), // height
        [1,0].to_vec(), // planes
        [8,0].to_vec(), // bit count
        [0,0,0,0].to_vec(), // compression
        [0,0,0,0].to_vec(), // image size (compressed)
        [0,0,0,0].to_vec(), // x pixels per meter
        [0,0,0,0].to_vec(), // y pixels per meter
        colors.to_le_bytes().to_vec(), // colors used
        [0,0,0,0].to_vec(), // colors important
        palette,
        raster.to_vec(),
    ].concat();

    bytes
}

fn lz77_uncomp (data: &Vec<u8>, addr: usize) -> (Vec<u8>,usize) {
    let mut src = addr;
    if data[src] != 0x10 {
        println!("not lz77 data");
        return (Vec::new(),0);
    }
    src += 1;
    let size = data[src] as usize
        + ((data[src+1] as usize)<<8)
        + ((data[src+2] as usize)<<16);
    src += 3;

    let mut out = vec![0; size];
    let mut dst = 0;

    while dst < size {
        let mut flags = data[src];
        src += 1;
        for _i in 0..8 {
            if flags&0x80 == 0 {
                out[dst] = data[src];
                dst += 1;
                src += 1;
            } else {
                let len = 3 + data[src] as usize / 0x10;
                let disp = 1 + (data[src] as usize &0xF)*0x100 + data[src+1] as usize;
                src += 2;

                for _j in 0..len {
                    out[dst] = out[dst-disp];
                    dst += 1;
                    if dst >= size {
                        break;
                    }
                }
            }
            flags = flags << 1;
            if dst >= size {
                break;
            }
        }
    }

    return (out,src-addr);
}

fn read_entries (data: &Vec<u8>, addr: usize, len: usize) -> Vec<Vec<u32>> {
    let mut out = Vec::new();

    for i in 0..len {
        let b = read_u32(data, (addr-0x800_0000+i*68) as usize, 17);
        println!("{:8X} {}: {:8X?}",addr+i*68,i,b);
        out.push(b);
    }

    println!(); println!();
    out
}

fn create_palette(data: &Vec<u8>, paddr: usize, pnum: i32) {
    let pdata = read_u8(&data, (paddr-0x800_0000) as usize, 0xC0*2);

    let _ = fs::write(format!("pal_{:02}.bin",pnum), pdata);

    let pdata2: Vec<u16> = [
        vec![0; 0x40],
        read_u16(&data, (paddr-0x800_0000) as usize, 0xC0)
        ].concat();

    let mut raster: Vec<u8> = vec![0; 256];
    (0..256).for_each(|i| raster[i]=i as u8);
    let pbmp = gba_to_bmp(&pdata2, &raster, 16, 16);
    let _ = fs::write(format!("pal_{:02}.bmp",pnum), pbmp);
}
