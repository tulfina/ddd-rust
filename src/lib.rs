mod aggregate;
mod command;
mod event;

#[cfg(test)]
mod tests {
    use crate::aggregate::Aggregate;
    use crate::command::Command;
    use crate::event::Event;
    use serde_json::Value;

    #[test]
    fn create_valid_command() {
        let command = Command::new(
            String::from("id"),
            String::from("type"),
            String::from("aggregate_id"),
            {
                let mut map = serde_json::Map::new();
                map.insert(String::from("key"), Value::from("value"));
                Value::Object(map)
            },
        );

        assert_eq!(command.id, "id");
        assert_eq!(command._type, "type");
        assert_eq!(command.aggregate_id, "aggregate_id");
        assert_eq!(command.payload, {
            let mut map = serde_json::Map::new();
            map.insert(String::from("key"), Value::from("value"));
            Value::Object(map)
        });
    }

    #[test]
    fn create_valid_event() {
        let command = Command::new(
            String::from("id"),
            String::from("type"),
            String::from("aggregate_id"),
            {
                let mut map = serde_json::Map::new();
                map.insert(String::from("key"), Value::from("value"));
                Value::Object(map)
            },
        );

        let event = Event::new(String::from("id"), String::from("type"), command, {
            let mut map = serde_json::Map::new();
            map.insert(String::from("key"), Value::from("value"));
            Value::Object(map)
        });

        assert_eq!(event.id, Some(String::from("id")));
        assert_eq!(event.type_, String::from("type"));
        assert_eq!(event.aggregate_id, String::from("aggregate_id"));
        assert_eq!(event.correlation_id, None);
        assert_eq!(event.payload, {
            let mut map = serde_json::Map::new();
            map.insert(String::from("key"), Value::from("value"));
            Value::Object(map)
        });
        assert_eq!(event.headers, None);
        assert_eq!(event.additional_information, None);
    }

    #[test]
    fn create_valid_aggregate() {
        struct State {
            key: String,
        }

        enum Commands {
            Register { key: String },
        }

        struct Events {
            Registered: Event<State>,
        }

        struct TestAggregate {
            state: State,
            register: fn(&mut Self, Commands::Register) -> Result<(), String>,
        }

        impl Aggregate<State> for TestAggregate {
            type Commands = Commands;
            type Events = Events;

            fn sink(&mut self, command: &Command<State>) -> Result<(), String> {}

            fn apply(&mut self, event: &Event<State>) -> Result<(), String> {
                Ok(())
            }

            fn state(&self) -> State {
                self.state
            }

            fn register(&mut self, payload: Commands::Register) -> Result<(), String> {
                let command = Command::new(
                    String::from("id"),
                    String::from("type"),
                    String::from("aggregate_id"),
                    {
                        let mut map = serde_json::Map::new();
                        map.insert(String::from("key"), Value::from("value"));
                        Value::Object(map)
                    },
                );
                self.sink(&command)?;
            }
        }

        let aggregate = TestAggregate { state: {} };
        aggregate.register(Commands::Register {
            key: String::from("value"),
        });

        assert_eq!(aggregate.state().key, String::from("value"));
    }
}
