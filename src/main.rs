use rand::{thread_rng, Rng};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::env;
use std::fs::read;
use std::ops::Index;
use std::time::{Duration, Instant};

const START_POS: usize = 0x200;
const SCALE: u32 = 15;

struct Keys {
    one: bool,
    two: bool,
    three: bool,
    four: bool,
    q: bool,
    w: bool,
    e: bool,
    r: bool,
    a: bool,
    s: bool,
    d: bool,
    f: bool,
    z: bool,
    x: bool,
    c: bool,
    v: bool,
}

impl Keys {
    fn new() -> Keys {
        Keys {
            one: false,
            two: false,
            three: false,
            four: false,
            q: false,
            w: false,
            e: false,
            r: false,
            a: false,
            s: false,
            d: false,
            f: false,
            z: false,
            x: false,
            c: false,
            v: false,
        }
    }

    fn check_key(&self, key: u8) -> bool {
        match key {
            0x0 => return self.one,
            0x1 => return self.two,
            0x2 => return self.three,
            0x3 => return self.four,
            0x4 => return self.q,
            0x5 => return self.w,
            0x6 => return self.e,
            0x7 => return self.r,
            0x8 => return self.a,
            0x9 => return self.s,
            0xa => return self.d,
            0xb => return self.f,
            0xc => return self.z,
            0xd => return self.x,
            0xe => return self.c,
            0xf => return self.v,
            default => return self.one,
        }
    }
}

struct Memory {
    memory: [u8; 4096],
}

impl Memory {
    fn new() -> Memory {
        Memory { memory: [0; 4096] }
    }
}

struct Cpu {
    V0: u8,
    V1: u8,
    V2: u8,
    V3: u8,
    V4: u8,
    V5: u8,
    V6: u8,
    V7: u8,
    V8: u8,
    V9: u8,
    VA: u8,
    VB: u8,
    VC: u8,
    VD: u8,
    VE: u8,
    VF: u8, // but its really a bit
    I: u16, // but its really u12
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            V0: 0,
            V1: 0,
            V2: 0,
            V3: 0,
            V4: 0,
            V5: 0,
            V6: 0,
            V7: 0,
            V8: 0,
            V9: 0,
            VA: 0,
            VB: 0,
            VC: 0,
            VD: 0,
            VE: 0,
            VF: 0,
            I: 0,
        }
    }

    fn set_add_reg(&mut self, value: u16) {
        self.I = value;
    }

    fn set_reg(&mut self, reg: u16, value: u8) {
        match reg {
            0x0 => self.V0 = value,
            0x1 => self.V1 = value,
            0x2 => self.V2 = value,
            0x3 => self.V3 = value,
            0x4 => self.V4 = value,
            0x5 => self.V5 = value,
            0x6 => self.V6 = value,
            0x7 => self.V7 = value,
            0x8 => self.V8 = value,
            0x9 => self.V9 = value,
            0xa => self.VA = value,
            0xb => self.VB = value,
            0xc => self.VC = value,
            0xd => self.VD = value,
            0xe => self.VE = value,
            0xf => self.VF = value,
            default => (),
        }
    }

    fn get_add_reg(&self) -> u16 {
        return self.I;
    }

    fn get_reg(&mut self, reg: u16) -> u8 {
        match reg {
            0x0 => return self.V0,
            0x1 => return self.V1,
            0x2 => return self.V2,
            0x3 => return self.V3,
            0x4 => return self.V4,
            0x5 => return self.V5,
            0x6 => return self.V6,
            0x7 => return self.V7,
            0x8 => return self.V8,
            0x9 => return self.V9,
            0xa => return self.VA,
            0xb => return self.VB,
            0xc => return self.VC,
            0xd => return self.VD,
            0xe => return self.VE,
            0xf => return self.VF,
            default => return self.V0,
        }
    }
}

struct Gfx {
    screen: [[u8; 32]; 64],
}

fn draw_on_canvas(x: i32, y: i32, canvas: &mut Canvas<Window>, color: u8) {
    if color == 0 {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
    }
    if color == 1 {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
    }
    //let point = Point::new(x * (SCALE as i32), y * (SCALE as i32));
    //canvas.draw_point(point);
    let rect = Rect::new(x * (SCALE as i32), y * (SCALE as i32), SCALE, SCALE);
    canvas.draw_rect(rect);
    canvas.fill_rect(rect);
}

impl Gfx {
    fn new() -> Gfx {
        Gfx {
            screen: [[0; 32]; 64],
        }
    }

    fn draw(&mut self, x: u8, y: u8, row: u8, canvas: &mut Canvas<Window>) -> bool {
        let mut collision = false;
        let masking = [
            0b1, 0b01, 0b001, 0b0001, 0b00001, 0b000001, 0b0000001, 0b00000001,
        ];

        for n in 0..8 {
            let screen_pixel = self.screen[wrapX(x, n) as usize][y as usize];
            self.screen[wrapX(x, n) as usize][y as usize] =
                screen_pixel ^ (row >> (7 - n) & masking[n as usize]);
            draw_on_canvas(
                wrapX(x, n) as i32,
                y as i32,
                canvas,
                self.screen[wrapX(x, n) as usize][y as usize],
            );
            if self.screen[wrapX(x, n) as usize][y as usize] == 0 && screen_pixel == 1 {
                collision = true;
            }
        }

        collision
    }
}

struct Stack {
    stack: Vec<usize>,
}

impl Stack {
    fn new() -> Stack {
        Stack { stack: Vec::new() }
    }
}

fn load_rom(memory: &mut Memory, filename: &String) {
    println!("filename {}", filename);
    let rom = read(filename).unwrap();
    for i in 0..rom.len() {
        memory.memory[START_POS + i] = rom[i];
    }
}

fn wrapX(x: u8, offset: u8) -> u8 {
    let new = x as i8 + offset as i8;
    if new > 63 {
        return (new % 64) as u8;
    } else if new < 0 {
        return (new.abs() % 64) as u8;
    }
    new as u8
}

fn wrapY(y: u8, offset: u8) -> u8 {
    let new = y as i8 + offset as i8;
    if new < 0 {
        return (new.abs() % 32) as u8;
    } else if new > 31 {
        return (new % 32) as u8;
    }
    new as u8
}

fn load_fonts(memory: &mut Memory) {
    memory.memory[0] = 0xf0; // 0
    memory.memory[1] = 0x90;
    memory.memory[2] = 0x90;
    memory.memory[3] = 0x90;
    memory.memory[4] = 0xf0;
    memory.memory[5] = 0x70; // 1
    memory.memory[6] = 0x20;
    memory.memory[7] = 0x20;
    memory.memory[8] = 0x60;
    memory.memory[9] = 0x20;
    memory.memory[10] = 0xf0; // 2
    memory.memory[11] = 0x80;
    memory.memory[12] = 0xf0;
    memory.memory[13] = 0x10;
    memory.memory[14] = 0xf0;
    memory.memory[15] = 0xf0; // 3
    memory.memory[16] = 0x10;
    memory.memory[17] = 0xf0;
    memory.memory[18] = 0x10;
    memory.memory[19] = 0xf0;
    memory.memory[20] = 0x10; // 4
    memory.memory[21] = 0x10;
    memory.memory[22] = 0xf0;
    memory.memory[23] = 0x90;
    memory.memory[24] = 0x90;
    memory.memory[25] = 0xf0; // 5
    memory.memory[26] = 0x10;
    memory.memory[27] = 0xf0;
    memory.memory[28] = 0x80;
    memory.memory[29] = 0xf0;
    memory.memory[30] = 0xf0; // 6
    memory.memory[31] = 0x90;
    memory.memory[32] = 0xf0;
    memory.memory[33] = 0x80;
    memory.memory[34] = 0xf0;
    memory.memory[35] = 0x40; // 7
    memory.memory[36] = 0x40;
    memory.memory[37] = 0x20;
    memory.memory[38] = 0x10;
    memory.memory[39] = 0xf0;
    memory.memory[40] = 0xf0; // 8
    memory.memory[41] = 0x90;
    memory.memory[42] = 0xf0;
    memory.memory[43] = 0x90;
    memory.memory[44] = 0xf0;
    memory.memory[45] = 0xf0; // 9
    memory.memory[46] = 0x10;
    memory.memory[47] = 0xf0;
    memory.memory[48] = 0x90;
    memory.memory[49] = 0xf0;
    memory.memory[50] = 0x90; // A
    memory.memory[51] = 0x90;
    memory.memory[52] = 0xf0;
    memory.memory[53] = 0x90;
    memory.memory[54] = 0xf0;
    memory.memory[55] = 0xe0; // B
    memory.memory[56] = 0x90;
    memory.memory[57] = 0xe0;
    memory.memory[58] = 0x90;
    memory.memory[59] = 0xe0;
    memory.memory[60] = 0xf0; // C
    memory.memory[61] = 0x80;
    memory.memory[62] = 0x80;
    memory.memory[63] = 0x80;
    memory.memory[64] = 0xf0;
    memory.memory[65] = 0xe0; // D
    memory.memory[66] = 0x90;
    memory.memory[67] = 0x90;
    memory.memory[68] = 0x90;
    memory.memory[69] = 0xe0;
    memory.memory[70] = 0xf0; // E
    memory.memory[71] = 0x80;
    memory.memory[72] = 0xf0;
    memory.memory[73] = 0x80;
    memory.memory[74] = 0xf0;
    memory.memory[75] = 0x80; // F
    memory.memory[76] = 0x80;
    memory.memory[77] = 0xf0;
    memory.memory[78] = 0x80;
    memory.memory[79] = 0xf0;
}

fn main() {
    let mut memory = Memory::new();
    let filename: Vec<String> = env::args().collect();
    load_rom(&mut memory, filename.index(1));
    load_fonts(&mut memory);
    main_loop(&mut memory);
}

fn main_loop(memory: &mut Memory) {
    let mut position = START_POS;
    let mut delay_timer = 0;
    let mut sound_timer = 0;
    let mut cpu = Cpu::new();
    let mut gfx = Gfx::new();
    let mut stack = Stack::new();
    let mut clock_count = 0;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("chip 8", 64 * SCALE, 32 * SCALE)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut keys = Keys::new();

    loop {
        let start = Instant::now();

        if clock_count >= 340 {
            clock_count = 0;
        }

        if clock_count % 2 == 0 {
            let mut event_pump = sdl_context.event_pump().unwrap();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => return,
                    Event::KeyDown {
                        keycode: Some(Keycode::Num1),
                        ..
                    } => {
                        keys.one = true;
                    }
                    Event::KeyUp {
                        keycode: Some(Keycode::Num1),
                        ..
                    } => {
                        keys.one = false;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Num2),
                        ..
                    } => {
                        keys.two = true;
                    }
                    Event::KeyUp {
                        keycode: Some(Keycode::Num2),
                        ..
                    } => {
                        keys.two = false;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Num3),
                        ..
                    } => {
                        keys.three = true;
                    }
                    Event::KeyUp {
                        keycode: Some(Keycode::Num3),
                        ..
                    } => {
                        keys.three = false;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Num4),
                        ..
                    } => {
                        keys.four = true;
                    }
                    Event::KeyUp {
                        keycode: Some(Keycode::Num4),
                        ..
                    } => {
                        keys.four = false;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::A),
                        ..
                    } => {
                        keys.a = true;
                    }
                    Event::KeyUp {
                        keycode: Some(Keycode::A),
                        ..
                    } => {
                        keys.a = false;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::S),
                        ..
                    } => {
                        keys.s = true;
                    }
                    Event::KeyUp {
                        keycode: Some(Keycode::S),
                        ..
                    } => {
                        keys.s = false;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::D),
                        ..
                    } => {
                        keys.d = true;
                    }
                    Event::KeyUp {
                        keycode: Some(Keycode::D),
                        ..
                    } => {
                        keys.d = false;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::F),
                        ..
                    } => {
                        keys.f = true;
                    }
                    Event::KeyUp {
                        keycode: Some(Keycode::F),
                        ..
                    } => {
                        keys.f = false;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Q),
                        ..
                    } => {
                        keys.q = true;
                    }
                    Event::KeyUp {
                        keycode: Some(Keycode::Q),
                        ..
                    } => {
                        keys.q = false;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::W),
                        ..
                    } => {
                        keys.w = true;
                    }
                    Event::KeyUp {
                        keycode: Some(Keycode::W),
                        ..
                    } => {
                        keys.w = false;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::E),
                        ..
                    } => {
                        keys.e = true;
                    }
                    Event::KeyUp {
                        keycode: Some(Keycode::E),
                        ..
                    } => {
                        keys.e = false;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::R),
                        ..
                    } => {
                        keys.r = true;
                    }
                    Event::KeyUp {
                        keycode: Some(Keycode::R),
                        ..
                    } => {
                        keys.r = false;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Z),
                        ..
                    } => {
                        keys.z = true;
                    }
                    Event::KeyUp {
                        keycode: Some(Keycode::Z),
                        ..
                    } => {
                        keys.z = false;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::X),
                        ..
                    } => {
                        keys.x = true;
                    }
                    Event::KeyUp {
                        keycode: Some(Keycode::X),
                        ..
                    } => {
                        keys.x = false;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::C),
                        ..
                    } => {
                        keys.c = true;
                    }
                    Event::KeyUp {
                        keycode: Some(Keycode::C),
                        ..
                    } => {
                        keys.c = false;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::V),
                        ..
                    } => {
                        keys.v = true;
                    }
                    Event::KeyUp {
                        keycode: Some(Keycode::V),
                        ..
                    } => {
                        keys.v = false;
                    }
                    _ => {}
                }
            }

            let opcode: u16 =
                (memory.memory[position] as u16) << 8 | (memory.memory[position + 1] as u16);

            //println!("{:#x}", position);
            //println!("{:#x}", opcode);
            //println!("--------------");

            position = execute_opcode(
                opcode,
                memory,
                &mut cpu,
                &mut gfx,
                &mut stack,
                &position,
                &mut delay_timer,
                &mut sound_timer,
                &mut canvas,
                &keys,
            );
        }

        if clock_count % 17 == 0 {
            if delay_timer > 0 {
                delay_timer -= 1;
            }
            if sound_timer > 0 {
                sound_timer -= 1;
            }
        }

        clock_count += 1;

        let duration = start.elapsed();
        let greatest_sleep = Duration::from_millis(1);
        if duration < greatest_sleep {
            let sleep_time = Duration::from_millis(1) - duration;
            std::thread::sleep(sleep_time);
        }
    }
}

fn execute_opcode(
    opcode: u16,
    memory: &mut Memory,
    cpu: &mut Cpu,
    gfx: &mut Gfx,
    stack: &mut Stack,
    position: &usize,
    delay_timer: &mut u8,
    sound_timer: &mut u8,
    canvas: &mut Canvas<Window>,
    keys: &Keys,
) -> usize {
    match opcode {
        0x00E0 => {
            gfx.screen = [[0; 32]; 64]; // clear screen
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();
            canvas.present();
            return position + 2;
        }
        0x00EE => {
            return stack.stack.pop().unwrap() as usize; // return from subroutine
        }
        default => {
            if (opcode >> 12) == 0x0 {
                // 0NNN
                // call code at given address
                // no longer used by chip 8
                return (opcode & 0x0fff) as usize;
            }
            if (opcode >> 12) == 0x1 {
                // 1NNN
                // jump to given address
                return (opcode & 0x0fff) as usize;
            }
            if (opcode >> 12) == 0x2 {
                // 2NNN
                // call subroutine at given address
                stack.stack.push(position + 2); // store next pc on stack
                return (opcode & 0x0fff) as usize;
            }
            if (opcode >> 12) == 0x3 {
                // 3XNN
                // if VX = NN -> skip the next instruction
                let reg_x_name = ((opcode >> 8) & 0x0f);
                if cpu.get_reg(reg_x_name) == ((opcode & 0x00ff) as u8) {
                    return position + 4;
                }
                return position + 2;
            }
            if (opcode >> 12) == 0x4 {
                // 4XNN
                // if VX != NN -> skip the next instruction
                let reg_x_name = (opcode >> 8) & 0x0f;
                if cpu.get_reg(reg_x_name) != ((opcode & 0x00ff) as u8) {
                    return position + 4;
                }
                return position + 2;
            }
            if ((opcode >> 12) == 0x5) && ((opcode & 0x000f) == 0x0000) {
                // 5XY0
                // if VX = VY -> skip next instruction
                let reg_x_name = (opcode >> 8) & 0x0f;
                let reg_y_name = (opcode >> 4) & 0x00f;
                if cpu.get_reg(reg_x_name) == cpu.get_reg(reg_y_name) as u8 {
                    return position + 4;
                }
                return position + 2;
            }
            if (opcode >> 12) == 0x6 {
                // 6XNN
                // set VX = NN
                let reg_name = (opcode >> 8) & 0x0f;
                let value = (opcode & 0x00ff) as u8;
                cpu.set_reg(reg_name, value);
                return position + 2;
            }
            if (opcode >> 12) == 0x7 {
                // 7XNN
                // set VX += NN
                let reg_x_name = (opcode >> 8) & 0x0f;
                let value = ((opcode & 0x00ff) as u16) + (cpu.get_reg(reg_x_name) as u16);
                cpu.set_reg(reg_x_name, value as u8);
                return position + 2;
            }
            if ((opcode >> 12) == 0x8) && ((opcode & 0x000f) == 0x0000) {
                // 8XY0
                // set VX = VY
                let reg_x_name = (opcode >> 8) & 0x0f;
                let reg_y_name = (opcode >> 4) & 0x00f;
                let value = cpu.get_reg(reg_y_name);
                cpu.set_reg(reg_x_name, value);
                return position + 2;
            }
            if ((opcode >> 12) == 0x8) && ((opcode & 0x000f) == 0x0001) {
                // 8XY1
                // set VX |= VY
                let reg_x_name = (opcode >> 8) & 0x0f;
                let reg_y_name = (opcode >> 4) & 0x00f;
                let value = cpu.get_reg(reg_x_name) | cpu.get_reg(reg_y_name);
                cpu.set_reg(reg_x_name, value);
                return position + 2;
            }
            if ((opcode >> 12) == 0x8) && ((opcode & 0x000f) == 0x0002) {
                // 8XY2
                // set VX &= VY
                let reg_x_name = (opcode >> 8) & 0x0f;
                let reg_y_name = (opcode >> 4) & 0x00f;
                let value = cpu.get_reg(reg_x_name) & cpu.get_reg(reg_y_name);
                cpu.set_reg(reg_x_name, value);
                return position + 2;
            }
            if ((opcode >> 12) == 0x8) && ((opcode & 0x000f) == 0x0003) {
                // 8XY3
                // set VX ^= VY
                let reg_x_name = (opcode >> 8) & 0x0f;
                let reg_y_name = (opcode >> 4) & 0x00f;
                let value = cpu.get_reg(reg_x_name) ^ cpu.get_reg(reg_y_name);
                cpu.set_reg(reg_x_name, value);
                return position + 2;
            }
            if ((opcode >> 12) == 0x8) && ((opcode & 0x000f) == 0x0004) {
                // 8XY4
                // set VX += VY w/ carry
                let reg_x_name = (opcode >> 8) & 0x0f;
                let reg_y_name = (opcode >> 4) & 0x00f;
                let value = (cpu.get_reg(reg_x_name) as u16) + (cpu.get_reg(reg_y_name) as u16);
                let carry = (value >= 0x100) as u8;
                cpu.set_reg(reg_x_name, (value & 0x00FF) as u8);
                cpu.set_reg(0xf, carry);
                return position + 2;
            }
            if ((opcode >> 12) == 0x8) && ((opcode & 0x000f) == 0x0005) {
                // 8XY5
                // set VX -= VY w/ borrow
                let reg_x_name = (opcode >> 8) & 0x0f;
                let reg_y_name = (opcode >> 4) & 0x00f;
                let value = (cpu.get_reg(reg_x_name) as i32) - (cpu.get_reg(reg_y_name) as i32);
                if value < 0 {
                    cpu.set_reg(reg_x_name, (0x00FF - (value.abs() & 0x00FF)) as u8);
                    cpu.set_reg(0xf, 0);
                } else {
                    cpu.set_reg(reg_x_name, value as u8);
                    cpu.set_reg(0xf, 1);
                }
                return position + 2;
            }
            if ((opcode >> 12) == 0x8) && ((opcode & 0x000f) == 0x0006) {
                // 8XY6
                // set VX >>= 1
                let reg_x_name = (opcode >> 8) & 0x0f;
                let reg_x_value = cpu.get_reg(reg_x_name);
                let bit = reg_x_value & 1;
                cpu.set_reg(reg_x_name, reg_x_value >> 1);
                cpu.set_reg(0xf, bit);
                return position + 2;
            }
            if ((opcode >> 12) == 0x8) && ((opcode & 0x000f) == 0x0007) {
                // 8XY7
                // set VX = VY - VX w/ borrow
                let reg_x_name = (opcode >> 8) & 0x0f;
                let reg_y_name = (opcode >> 4) & 0x00f;
                let value = (cpu.get_reg(reg_y_name) as i32) - (cpu.get_reg(reg_x_name) as i32);
                if value < 0 {
                    cpu.set_reg(reg_x_name, (0xFF - (value.abs() & 0x00FF)) as u8);
                    cpu.set_reg(0xf, 0);
                } else {
                    cpu.set_reg(reg_x_name, value as u8);
                    cpu.set_reg(0xf, 1);
                }
                return position + 2;
            }
            if ((opcode >> 12) == 0x8) && ((opcode & 0x000f) == 0x000E) {
                // 8XYE
                // set VX <<= 1
                let reg_x_name = (opcode >> 8) & 0x0f;
                let reg_x_value = cpu.get_reg(reg_x_name);
                let bit = reg_x_value >> 7;
                cpu.set_reg(reg_x_name, reg_x_value << 1);
                cpu.set_reg(0xf, bit);
                return position + 2;
            }
            if ((opcode >> 12) == 0x9) && ((opcode & 0x000f) == 0x0000) {
                // 9XY0
                // if VX != VY -> skip next instruction
                let reg_x_name = (opcode >> 8) & 0x0f;
                let reg_y_name = (opcode >> 4) & 0x00f;
                if cpu.get_reg(reg_x_name) != cpu.get_reg(reg_y_name) {
                    return position + 4;
                }
                return position + 2;
            }
            if (opcode >> 12) == 0xA {
                // ANNN
                // set I = NNN
                let address = opcode & 0x0fff;
                cpu.set_add_reg(address);
                return position + 2;
            }
            if (opcode >> 12) == 0xB {
                // BNNN
                // jump to NNN + V0
                return ((opcode & 0x0fff) + (cpu.get_reg(0x0) as u16)) as usize;
            }
            if (opcode >> 12) == 0xC {
                // CXNN
                // VX = rand() & NN
                let reg_name = (opcode >> 8) & 0x0f;
                let random = thread_rng().gen_range(0..256);
                cpu.set_reg(reg_name, ((opcode & 0x00ff) & random) as u8);
                return position + 2;
            }
            if (opcode >> 12) == 0xD {
                // DXYN
                // Draw sprite at VX,VY of height N
                // Set VF is a collision occurs
                let x = cpu.get_reg((opcode >> 8) & 0x0f);
                let y = cpu.get_reg((opcode >> 4) & 0x00f);
                let height = opcode & 0x000f;
                let start = cpu.get_add_reg();
                let mut collision = false;
                for n in 0..height {
                    let row = memory.memory[(start + n) as usize];
                    if gfx.draw(x, wrapY(y, (n as u8)), row, canvas) {
                        collision = true;
                    }
                }
                if collision {
                    cpu.set_reg(0xf, 1);
                } else {
                    cpu.set_reg(0xf, 0);
                }
                canvas.present();
                return position + 2;
            }
            if ((opcode >> 12) == 0xE) && ((opcode & 0x00ff) == 0x009E) {
                // EX9E
                // if keypress == VX -> skip next instruction
                let reg_name = (opcode >> 8) & 0x0f;
                let key_value = cpu.get_reg(reg_name);
                if keys.check_key(key_value) {
                    return position + 4;
                }
                return position + 2;
            }
            if ((opcode >> 12) == 0xE) && ((opcode & 0x00ff) == 0x00A1) {
                // EXA1
                // if keypress != VX -> skip next instruction
                let reg_name = (opcode >> 8) & 0x0f;
                let key_value = cpu.get_reg(reg_name);
                if !keys.check_key(key_value) {
                    return position + 4;
                }
                return position + 2;
            }
            if ((opcode >> 12) == 0xF) && ((opcode & 0x00ff) == 0x0007) {
                // FX07
                // VX = delay_timer
                let reg_name = (opcode >> 8) & 0x0f;
                cpu.set_reg(reg_name, *delay_timer);
                return position + 2;
            }
            if ((opcode >> 12) == 0xF) && ((opcode & 0x00ff) == 0x000A) {
                // FX0A
                // VX = await keypress
                let keypress = 0x0; // block execution and get input from hex keyboard
                let reg_name = (opcode >> 8) & 0x0f;
                cpu.set_reg(reg_name, keypress);
                return position + 2;
            }
            if ((opcode >> 12) == 0xF) && ((opcode & 0x00ff) == 0x0015) {
                // FX15
                // delay_timer = VX
                let reg_name = (opcode >> 8) & 0x0f;
                *delay_timer = cpu.get_reg(reg_name);
                return position + 2;
            }
            if ((opcode >> 12) == 0xF) && ((opcode & 0x00ff) == 0x0018) {
                // FX18
                // sound_timer = VX
                let reg_name = (opcode >> 8) & 0x0f;
                *sound_timer = cpu.get_reg(reg_name);
                return position + 2;
            }
            if ((opcode >> 12) == 0xF) && ((opcode & 0x00ff) == 0x001E) {
                // FX1E
                // I += VX
                let reg_name = (opcode >> 8) & 0x0f;
                let I = cpu.get_add_reg();
                let VX = cpu.get_reg(reg_name) as u16;
                cpu.set_add_reg(I + VX);
                return position + 2;
            }
            if ((opcode >> 12) == 0xF) && ((opcode & 0x00ff) == 0x0029) {
                // FX29
                // set I to location of font stored in VX
                let reg_name = (opcode >> 8) & 0x0f;
                let letter = cpu.get_reg(reg_name);
                let letter_position = match letter {
                    0x0 => 0,
                    0x1 => 5,
                    0x2 => 10,
                    0x3 => 15,
                    0x4 => 20,
                    0x5 => 25,
                    0x6 => 30,
                    0x7 => 35,
                    0x8 => 40,
                    0x9 => 45,
                    0xa => 50,
                    0xb => 55,
                    0xc => 60,
                    0xd => 65,
                    0xe => 70,
                    0xf => 75,
                    default => 0,
                };
                cpu.set_add_reg(letter_position);
                return (position + 2) as usize;
            }
            if ((opcode >> 12) == 0xF) && ((opcode & 0x00ff) == 0x0033) {
                // FX33
                // store binary coded decimal of VX in I
                let reg_name = (opcode >> 8) & 0x0f;
                let address = cpu.get_add_reg();
                let value = cpu.get_reg(reg_name);
                memory.memory[address as usize] = (value / 100) as u8;
                memory.memory[(address + 1) as usize] = ((value / 10) % 10) as u8;
                memory.memory[(address + 2) as usize] = (value % 10) as u8;
                return position + 2;
            }
            if ((opcode >> 12) == 0xF) && ((opcode & 0x00ff) == 0x0055) {
                // FX55
                // Load V0-VX into I+
                let reg_name = (opcode >> 8) & 0x0f;
                let mut start_reg = 0x0 as u16;
                let mut start_address = cpu.get_add_reg();
                while start_reg <= reg_name {
                    memory.memory[start_address as usize] = cpu.get_reg(start_reg);
                    start_reg += 1;
                    start_address += 1;
                }
                return position + 2;
            }
            if ((opcode >> 12) == 0xF) && ((opcode & 0x00ff) == 0x0065) {
                // FX65
                // Reads V0-VX from I+
                let reg_name = (opcode >> 8) & 0x0f;
                let mut start_reg = 0x0 as u16;
                let mut start_address = cpu.get_add_reg();
                while start_reg <= reg_name {
                    cpu.set_reg(start_reg, memory.memory[start_address as usize]);
                    start_reg += 1;
                    start_address += 1;
                }
                return position + 2;
            } else {
                panic!("unknown opcode {}", opcode);
            }
        }
    }
}
