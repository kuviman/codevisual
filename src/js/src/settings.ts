namespace CodeVisual {
    interface Setting {
        addTo(parent: JQuery): void;
    }
    export class BooleanSetting implements Setting {
        private _value: boolean;

        constructor(public name: string, defaultValue: boolean, private setter?: (newValue: boolean) => void) {
            this.value = defaultValue;
        }

        get value(): boolean {
            return this._value;
        }
        set value(newValue: boolean) {
            this._value = newValue;
            if (this.setter) {
                this.setter(newValue);
            }
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
    export class NumberSetting implements Setting {
        private _value: number;

        constructor(public name: string,
            private minValue: number, private maxValue: number, defaultValue: number,
            private step: number = 1, private setter?: (newValue: number) => void) {
            this.value = defaultValue;
        }

        get value(): number {
            return this._value;
        }
        set value(newValue: number) {
            this._value = newValue;
            if (this.setter) {
                this.setter(newValue);
            }
        }
        addTo(parent: JQuery) {
            const $setting = $player.find(".setting-template-number").clone().removeClass("setting-template-number").appendTo(parent);
            $setting.find(".name").text(this.name);
            const $input = $setting.find("input");

            $input.attr("min", this.minValue).attr("max", this.maxValue).attr("step", this.step).val(this.value).change(() => {
                this.value = $input.val();
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