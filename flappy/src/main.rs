use bracket_lib::prelude::*;


enum GameMode {
    Menu,     // 菜单模式
    Playing,  // 游戏中模式
    End,      // 结束模式
}

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0; // 每隔 75 毫秒就做一些事，如果小于这个时间就不做任何事了

 
struct Player {
    x: i32,        // 水平位置，游戏的世界空间的横坐标
    y: i32,        // 垂直位置
    velocity: f32, // 垂直方向的速度，速度 > 0 它就往下掉
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Player { 
            x: 0, 
            y: 0, 
            velocity: 0.0 // 速度用 f32 为了让其移动更丝滑
        }
    }
    // 渲染
    fn render(&mut self, ctx: &mut BTerm) {
        /*
            set 可以在屏幕设置字符，这里设置了 @ 这个字符，它来表示这个 player
            把 @ 这个字符转为 cp437 显示在屏幕上
         */
        ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'))

    }

    fn gravity_and_move(&mut self) {
        
        if self.velocity < 2.0 {
            self.velocity += 0.2; // 修改纵向速度
        }
        self.y += self.velocity as i32;
        self.x += 1;    // 每次调用这个函数，向右移动 1

        if self.y < 0 {    // 如果跑到上边去了 就等于 0 
            self.y = 0;
        }
    }

    // 按空格时候，往上飞
    fn flap(&mut self) {
        // 往上飞，速度就是负数
        self.velocity = -2.0;

    }
    
}

// 用于保存游戏状态
struct State {
    player: Player,
    frame_time: f32, // 表示经过了多少帧，累积了多少时间
    mode: GameMode,
    obstacle: Obstacle, // 障碍物
    score: i32,         // 积分
}

impl State {
    fn new() -> Self {
        State { 
            player: Player::new(5, 25),
            frame_time: 0.0,
            mode: GameMode::Menu,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            score: 0,
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
        // 清理屏幕，并指定背景颜色
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }

        // 按空格让龙往上飞
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }

        self.player.render(ctx);
        ctx.print(0, 0, "Press Space to Flap");
        ctx.print(0, 1, &format!("Score: {}", self.score));

        self.obstacle.render(ctx, self.player.x);

        if self.player.x > self.obstacle.x { // 如果玩家横坐标大于障碍物横坐标说明可以给分了
            self.score += 1;
            // 然后重新生成一个障碍物
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
        }
        if self.player.y > SCREEN_HEIGHT || self.obstacle.hit_obstacle(&self.player) {
            self.mode = GameMode::End;
        }
    }

    fn restart(&mut self) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
        self.score = 0;
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        // 在水平中间位置打印，只需要指定 y 的坐标
        ctx.print_centered(5, "You are dead!");
        ctx.print_centered(6, &format!("You earned {} points", self.score));
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


// 障碍 struct
struct Obstacle {
    x: i32,      // 游戏的世界空间的横坐标
    gap_y: i32,  // 中间的洞的纵坐标
    size: i32,   // 中间的洞的大小
}

impl Obstacle {
    // score 为玩家积分
    fn new(x: i32, score: i32) -> Self {
        // 伪随机数
        let mut random = RandomNumberGenerator::new();
        Obstacle { 
            x, 
            gap_y: random.range(10, 40), // 将障碍的中心点放到一个随机的位置, [10, 40) 范围
            size: i32::max(2, 20 - score), // 玩家积分越高，洞越窄，但最小是第一个参数 2，
        }
    }
    
    fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
        // 屏幕空间的横坐标，这里需要减去 player 的横坐标才行
        let screen_x = self.x - player_x;     
        let half_size = self.size / 2;

        for y in 0..self.gap_y - half_size {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }

        for y in self.gap_y + half_size..SCREEN_HEIGHT {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }

    }

    fn hit_obstacle(&self, player: &Player) -> bool {
        let half_size = self.size / 2;
        // 判断玩家的横坐标是否与障碍物的横坐标一致？
        // 如果一致，则有可能发生撞击
        let does_x_match = player.x == self.x;
        let player_above_gap = player.y < self.gap_y - half_size;
        let player_below_gap = player.y > self.gap_y + half_size;

        does_x_match && (player_above_gap || player_below_gap)
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
