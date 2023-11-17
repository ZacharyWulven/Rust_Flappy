use bracket_lib::prelude::*;


enum GameMode {
    Menu,     // 菜单模式
    Playing,  // 游戏中模式
    End,      // 结束模式
}

// 用于保存游戏状态
struct State {
    mode: GameMode,
}

impl State {
    fn new() -> Self {
        State { 
            mode: GameMode::Menu,
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        // 在水平中间位置打印，只需要指定 y 的坐标
        ctx.print_centered(5, "Welcom to Flappy Dragon");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");
        
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true, // 退出游戏
                _ => {},  // 其他的忽略
            }
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        self.mode = GameMode::End;
    }

    fn restart(&mut self) {
        self.mode = GameMode::Playing;

    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        // 在水平中间位置打印，只需要指定 y 的坐标
        ctx.print_centered(5, "You are dead!");
        ctx.print_centered(8, "(P) Play Again");
        ctx.print_centered(9, "(Q) Quit Game");
        
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true, // 退出游戏
                _ => {},  // 其他的忽略
            }
        }
    }
}


// 实现 trait 以便与 game loop 的 tick 函数关联上
impl GameState for State {
    /*
        ctx 就是上下文，相当于游戏的窗口类型
        可以对窗口进行清理屏幕，打印信息或捕获鼠标、键盘操作等
     */
    fn tick(&mut self, ctx: &mut BTerm) {
        // ctx.cls(); // 清理屏幕，通常是游戏的一般操作
        // // 打印东西到窗口上，坐标系从屏幕左上角开始
        // ctx.print(1, 1, "Hello, Bracket Terminal!");

        /*
            tick 函数，根据当前游戏的状态来指示这个游戏应该往哪个方向走
            所以我们要判断下游戏当前的状态
            使用 match 进行判断
         */
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.dead(ctx),
        }

    }
}

fn main() -> BError {
    // 创建一个 80x50 的窗口，其标题是 Flappy Dragon
    // 最后的 ? ，表示创建时可能出错，如果出错就返回 BError，否则创建成功
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;

    main_loop(context, State::new())
}
