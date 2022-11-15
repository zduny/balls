pub mod server {
    use serde::{Deserialize, Serialize};

    use crate::Color;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Message {
        Init {
            id: usize,
            color: Color,
        },
        Ball {
            id: usize,
            color: Color,
            position: (i32, i32),
        },
        Update {
            id: usize,
            position: (i32, i32),
        },
        Disconnected {
            id: usize,
        },
    }
}

pub mod client {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    pub struct Message {
        pub position: (i32, i32),
    }
}
