namespace CodeVisual {
    interface Setting {
        addTo(parent: JQuery): void;
    }
    export class BooleanSetting implements Setting {
        private _value: boolean;

        constructor(public name: string, defaultValue: boolean, private setter: (newValue: boolean) => void) {
            this.value = defaultValue;
        }

        get value(): boolean {
            return this._value;
        }
        set value(newValue: boolean) {
            this._value = newValue;
            this.setter(newValue);
        }
        addTo(parent: JQuery) {
            const $setting = $player.find(".setting-template-boolean").clone().removeClass("setting-template-boolean").appendTo(parent);
            $setting.find(".name").text(this.name);
            const $input = $setting.find("input");
            $input.prop("checked", this.value).change(() => {
                this.value = $input.is(":checked");
            });
        }
    }
    class Settings {
        add(setting: Setting) {
            setting.addTo($settingsTable);
        }
    }
    export const settings = new Settings();
    let $settings: JQuery;
    let $settingsTable: JQuery;

    internal.on_init.push(() => {
        $settings = $player.find(".settings");
        $settingsTable = $settings.find("table");
        $player.find(".settings-button").click(() => {
            $settings.slideToggle();
        });
    });
}