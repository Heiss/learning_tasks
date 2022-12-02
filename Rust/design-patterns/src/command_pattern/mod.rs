mod ceilingfan;
mod command;
mod garage;
mod light;
mod remote;

#[cfg(test)]
mod test_super {
    use std::{cell::RefCell, rc::Rc};

    use super::ceilingfan::{
        CeilingFan, CeilingFanHighCommand, CeilingFanLowCommand, CeilingFanMediumCommand,
        CeilingFanOffCommand,
    };

    use super::{
        garage::{GarageDoor, GarageDoorCloseCommand, GarageDoorOpenCommand},
        light::{Light, LightOffCommand, LightOnCommand},
        remote::{RemoteControl, SimpleRemoteControl},
    };

    #[test]
    fn test_simpleremotecontrol() {
        let mut remote: SimpleRemoteControl = SimpleRemoteControl::new();

        let light = Light::new("Living Room");
        let light_on = LightOnCommand::new(Rc::new(light));

        remote.set_command(Box::new(light_on));
        remote.button_was_pressed();

        let garage_door = GarageDoor::new();
        let garage_door_up = GarageDoorOpenCommand::new(Rc::new(garage_door));

        remote.set_command(Box::new(garage_door_up));
        remote.button_was_pressed();
    }

    #[test]
    fn test_remotecontrol() {
        let mut remote_control: RemoteControl = RemoteControl::new();

        let light = Rc::new(Light::new("Living Room"));

        let light_on = LightOnCommand::new(light.clone());
        let light_off = LightOffCommand::new(light);

        let garage_door = Rc::new(GarageDoor::new());
        let garage_door_up = GarageDoorOpenCommand::new(garage_door.clone());
        let garage_door_down = GarageDoorCloseCommand::new(garage_door);

        let ceiling_fan = Rc::new(RefCell::new(CeilingFan::new("Living Room")));
        let ceiling_fan_high = CeilingFanHighCommand::new_cmd(ceiling_fan.clone());
        let ceiling_fan_medium = CeilingFanMediumCommand::new_cmd(ceiling_fan.clone());
        let ceiling_fan_low = CeilingFanLowCommand::new_cmd(ceiling_fan.clone());
        let ceiling_fan_off = Rc::new(RefCell::new(CeilingFanOffCommand::new_cmd(ceiling_fan)));

        remote_control.set_command(
            0,
            Rc::new(RefCell::new(light_on)),
            Rc::new(RefCell::new(light_off)),
        );
        remote_control.set_command(
            1,
            Rc::new(RefCell::new(garage_door_up)),
            Rc::new(RefCell::new(garage_door_down)),
        );
        remote_control.set_command(
            2,
            Rc::new(RefCell::new(ceiling_fan_high)),
            ceiling_fan_off.clone(),
        );
        remote_control.set_command(
            3,
            Rc::new(RefCell::new(ceiling_fan_medium)),
            ceiling_fan_off.clone(),
        );
        remote_control.set_command(4, Rc::new(RefCell::new(ceiling_fan_low)), ceiling_fan_off);

        print!("{}", remote_control);

        remote_control.undo_button_was_pushed();
        remote_control.on_button_was_pushed(0);

        remote_control.undo_button_was_pushed();
        remote_control.undo_button_was_pushed();

        remote_control.off_button_was_pushed(0);
        remote_control.on_button_was_pushed(1);
        remote_control.off_button_was_pushed(1);
        remote_control.on_button_was_pushed(2);
        remote_control.off_button_was_pushed(2);
        remote_control.on_button_was_pushed(3);
        remote_control.off_button_was_pushed(3);
        remote_control.on_button_was_pushed(4);
        remote_control.off_button_was_pushed(4);

        remote_control.undo_button_was_pushed();
        remote_control.undo_button_was_pushed();
        remote_control.undo_button_was_pushed();
    }
}
