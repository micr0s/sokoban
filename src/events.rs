pub type EntityId = u32;

#[derive(Debug)]
pub struct LevelStart {
    pub level: u8,
}

#[derive(Debug)]
pub struct EntityMoved {
    pub id: EntityId,
}

#[derive(Debug)]
pub struct BoxPlacedOnSpot {
    pub is_correct_spot: bool,
}

#[derive(Debug)]
pub enum Event {
    // Fired when game is started
    LevelStart(LevelStart),

    // Fired when the player hits an obstacle like a wall
    PlayerHitObstacle,

    // Fired when all boxes placed on correct colour spots
    PlayerWon,

    // Fired when an entity is moved
    EntityMoved(EntityMoved),

    // Fired when the box is placed on a spot
    BoxPlacedOnSpot(BoxPlacedOnSpot),
}