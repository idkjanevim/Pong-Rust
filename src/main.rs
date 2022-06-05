use bevy::prelude::*;
use bevy::window;
use rand::thread_rng;
use rand::Rng;


const CLEAR: Color = Color::rgb(0.,0.,0.);
const PADDLE_WIDTH: f32 = 30.;
const PADDLE_HEIGHT: f32 = 150.;
const TIME_STEP: f32  = 1. / 60.;
const BALL_SPEED: f32 = 400.;


#[derive(Component)]
struct Player1;
#[derive(Component)]
struct Player2;
#[derive(Component)]
struct Ball;
#[derive(Component)]
struct Scoreboard{
    p1:usize,
    p2:usize
}
#[derive(Component)]
struct PositionOfPlayers{
    p1:Vec2,
    p2:Vec2
}

#[derive(Component)]
pub struct BallSpeed(Vec2);
impl Default for BallSpeed{
    fn default() -> Self {
        Self(randomize_vector(BALL_SPEED, BALL_SPEED))
    }
}
#[derive(Component)]
struct MoveSpeed(f32);
impl Default for MoveSpeed{
    fn default() -> Self {
        Self(500.)
    }
}

struct WinSize{
    w:f32,
    h:f32
}

fn main(){
    App::new()
        .insert_resource(window::WindowDescriptor {
            title: "Ponk!".to_string(),
            present_mode: window::PresentMode::Fifo,
            resizable:  false,
            ..Default::default()
            })
        .insert_resource(ClearColor(CLEAR))
        .add_startup_system(setup)
        .add_startup_stage("game_setup_actor",SystemStage::single(spawn_entities))
        .add_system(player1_move)
        .add_system(player2_move)
        .add_system(ball_move)
        .add_system(update_scoreboard)
        .insert_resource(Scoreboard{p1:0,p2:0})
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup(mut commands: Commands,mut windows: ResMut<Windows>){
    let camera = OrthographicCameraBundle::new_2d();
    let window = windows.get_primary_mut().unwrap();

    commands.insert_resource(WinSize{
        w:window.width(),
        h:window.height()
    });
    commands.insert_resource(PositionOfPlayers{
        p1:Vec2::new(0., 0.),
        p2:Vec2::new(0., 0.)
    });
    commands.spawn_bundle(UiCameraBundle::default());

    commands.spawn_bundle(camera);
}

fn spawn_entities(mut commands: Commands, winsize: Res<WinSize>,asset_server: Res<AssetServer>){
    //Player 1
    commands.spawn_bundle(SpriteBundle{ 
        sprite:Sprite{
            custom_size: Some(Vec2::new(PADDLE_WIDTH,PADDLE_HEIGHT)),
            color: Color::WHITE,
            ..Default::default()
        },
        transform:Transform{
            translation: Vec3::new(-winsize.w/2.+PADDLE_WIDTH/2.+10.,0.,10.),
            ..Default::default()
        },
        ..Default::default()
    }).insert(Player1).insert(MoveSpeed::default());

    //Player 2
    commands.spawn_bundle(SpriteBundle{ 
        sprite:Sprite{
            custom_size: Some(Vec2::new(PADDLE_WIDTH,PADDLE_HEIGHT)),
            color: Color::WHITE,
            ..Default::default()
        },
        transform:Transform{
            translation: Vec3::new(winsize.w/2.-PADDLE_WIDTH/2.-10.,0.,10.),
            ..Default::default()
        },
        ..Default::default()
    }).insert(Player2).insert(MoveSpeed::default());

    //Ball
    commands.spawn_bundle(SpriteBundle{ 
        sprite:Sprite{
            custom_size: Some(Vec2::new(30.,30.)),
            color: Color::WHITE,
            ..Default::default()
        },
        transform:Transform{
            translation: Vec3::new(0.,0.,0.),
            ..Default::default()
        },
        ..Default::default()
    }).insert(Ball).insert(BallSpeed::default());
    //Scoreboard
    commands.spawn_bundle(NodeBundle{
        style: Style{
            size: Size::new(Val::Percent(100.),Val::Percent(100.)),
            position_type:PositionType::Absolute,
            justify_content:JustifyContent::Center,
            align_items: AlignItems::FlexEnd,
            ..Default::default()
        },
        color:Color::Rgba { red: 1., green: 1., blue: 1., alpha: 0. }.into(),
        ..Default::default()
    }).with_children(|parent|{
        parent.spawn_bundle(TextBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "                        ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.,
                            color: Color::WHITE,
                        },
                    },
                ],
                ..default()
            },
            style: Style {
                ..default()
            },
            ..default()
        });
    });
}

fn randomize_vector(x: f32, y: f32) -> Vec2 {
    let mut vec = Vec2::new(0., 0.);
    let mut rng = thread_rng();
    vec.x = match rng.gen_bool(0.5) {
        true => x,
        false => -x,
    };
    vec.y = match rng.gen_bool(0.5) {
        true => y,
        false => -y,
    };
    vec
}

fn clamp(winsize: Res<WinSize>,transform: &mut Mut<Transform>,offset: f32){
    if transform.translation.y > winsize.h / 2. - offset / 2.{
        transform.translation.y = winsize.h / 2. - offset / 2.;
    }else if transform.translation.y < -winsize.h / 2. + offset / 2.{
        transform.translation.y = -winsize.h / 2. + offset / 2.;
    }
}

fn player1_move(input:Res<Input<KeyCode>>,mut query: Query<(&MoveSpeed,&mut Transform,With<Player1>)>,wins: Res<WinSize>,mut positions: ResMut<PositionOfPlayers>){
    let (speed, mut transform, _) = query.single_mut();

    let dir = if input.pressed(KeyCode::W){
        1.
    }else if input.pressed(KeyCode::S){
        -1.
    }else{
        0.
    };
    transform.translation.y += dir * speed.0 * TIME_STEP;
    clamp(wins,&mut transform,PADDLE_HEIGHT);
    positions.p1.x = transform.translation.x;
    positions.p1.y = transform.translation.y;
}

fn player2_move(input:Res<Input<KeyCode>>,mut query: Query<(&MoveSpeed,&mut Transform,With<Player2>)>,wins: Res<WinSize>,mut positions: ResMut<PositionOfPlayers>){
    let (speed, mut transform, _) = query.single_mut();

    let dir = if input.pressed(KeyCode::Up){
        1.
    }else if input.pressed(KeyCode::Down){
        -1.
    }else{
        0.
    };
    transform.translation.y += dir * speed.0 * TIME_STEP;
    clamp(wins,&mut transform,PADDLE_HEIGHT);
    positions.p2.x = transform.translation.x;
    positions.p2.y = transform.translation.y;
}

fn ball_move(mut query: Query<(&mut BallSpeed,&mut Transform,With<Ball>)>,wins: Res<WinSize>,mut scoreboard: ResMut<Scoreboard>,positions: Res<PositionOfPlayers>){
    let (mut speed, mut transform, _) = query.single_mut();

    transform.translation.x += speed.0[0] * TIME_STEP;
    transform.translation.y += speed.0[1] * TIME_STEP;
    //Check if ceiling is hit
    if transform.translation.y > wins.h / 2. - 30. / 2.{
        speed.0 = Vec2::new(speed.0[0],-speed.0[1]);
    }else if transform.translation.y < -wins.h / 2. + 30. / 2.{
        speed.0 = Vec2::new(speed.0[0],-speed.0[1]);
    }
    
    //Ball cords
    let ball_rcorner_x:f32 = transform.translation.x + 30./2.;  
    let ball_lcorner_x:f32 = transform.translation.x - 30./2.;
    
    let ball_upper_corner_y:f32 = transform.translation.y + 30./2.;
    let ball_lower_corner_y:f32 = transform.translation.y - 30./2.;
    //Player1 cords
    let p1_x:f32 = positions.p1.x + PADDLE_WIDTH/2.;

    let p1_upper_y:f32 = positions.p1.y + PADDLE_HEIGHT/2.;
    let p1_lower_y:f32 = positions.p1.y - PADDLE_HEIGHT/2.;
    //Player2 cords
    let p2_x:f32 = positions.p2.x - PADDLE_WIDTH/2.;
    let p2_upper_y:f32 = positions.p2.y + PADDLE_HEIGHT/2.;
    let p2_lower_y:f32 = positions.p2.y - PADDLE_HEIGHT/2.;


    

    //Check if paddle hit Player1
    if ball_lcorner_x < p1_x && ball_upper_corner_y < p1_upper_y && ball_upper_corner_y > p1_lower_y{
        speed.0 = Vec2::new(-speed.0[0],speed.0[1]);
    }else if ball_lcorner_x < p1_x && ball_lower_corner_y < p1_upper_y && ball_lower_corner_y > p1_lower_y{
        speed.0 = Vec2::new(-speed.0[0],speed.0[1]);
    }
    //Check if paddle hit Player2
    if ball_rcorner_x > p2_x && ball_upper_corner_y < p2_upper_y && ball_upper_corner_y > p2_lower_y{
        speed.0 = Vec2::new(-speed.0[0],speed.0[1]);
    }else if ball_rcorner_x > p2_x && ball_lower_corner_y < p2_upper_y && ball_lower_corner_y > p2_lower_y{
        speed.0 = Vec2::new(-speed.0[0],speed.0[1]);
    }
    //Check if point is scored
    if transform.translation.x > wins.w / 2. -30./2.{
        speed.0 = Vec2::new(-speed.0[0],speed.0[1]);
        scoreboard.p1 += 1;
        speed.0 = randomize_vector(BALL_SPEED, BALL_SPEED);
        transform.translation.x = 0.;
        transform.translation.y = 0.;

    }else if transform.translation.x < -wins.w / 2. + 30. / 2.{
        speed.0 = Vec2::new(-speed.0[0],speed.0[1]);
        scoreboard.p2 += 1;
                speed.0 = randomize_vector(BALL_SPEED, BALL_SPEED);
        transform.translation.x = 0.;
        transform.translation.y = 0.;
    }
}
fn update_scoreboard(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[0].value = format!("{}", scoreboard.p1);
    text.sections[2].value = format!("{}", scoreboard.p2);
}