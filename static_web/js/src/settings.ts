namespace CodeVisual {
    export interface Setting {
        addTo(parent: JQuery): void;
    }

    export class BooleanSetting implements Setting {
        private _value: boolean;

        constructor(public name: string,
                    private defaultValue: boolean,
                    private setter?: (newValue: boolean) => void) {
            let savedValue = window.localStorage.getItem(name);
            if (savedValue == null) {
                this.value = defaultValue;
            } else {
                this.value = savedValue == "true";
            }
        }

        get value(): boolean {
            return this._value;
        }

        set value(newValue: boolean) {
            this._value = newValue;
            if (this.setter) {
                this.setter(newValue);
            }
            window.localStorage.setItem(this.name, newValue.toString());
        }

        addTo(parent: JQuery) {
            const $setting = $player.find(".setting-template-boolean").clone().removeClass("setting-template-boolean").appendTo(parent);
            $setting.find(".name").text(this.name);
            const $input = $setting.find("input");
            $input.prop("checked", this.value).change(() => {
                this.value = $input.is(":checked");
            });
            $setting.find(".reset").click(() => {
                this.value = this.defaultValue;
                $input.prop("checked", this.value);
            });
        }
    }

    export class NumberSetting implements Setting {
        private _value: number;

        constructor(public name: string,
                    private minValue: number, private maxValue: number,
                    private defaultValue: number,
                    private step: number = 1,
                    private setter?: (newValue: number) => void) {
            let savedValue = window.localStorage.getItem(name);
            if (savedValue == null) {
                this.value = defaultValue;
            } else {
                this.value = parseFloat(savedValue);
            }
        }

        get value(): number {
            return this._value;
        }

        set value(newValue: number) {
            this._value = newValue;
            if (this.setter) {
                this.setter(newValue);
            }
            window.localStorage.setItem(this.name, newValue.toString());
        }

        addTo(parent: JQuery) {
            const $setting = $player.find(".setting-template-number").clone().removeClass("setting-template-number").appendTo(parent);
            $setting.find(".name").text(this.name);
            const $input = $setting.find("input");

            $input.attr("min", this.minValue).attr("max", this.maxValue).attr("step", this.step).val(this.value);
            $input.change(() => {
                this.value = $input.val();
            });
            $input.on("input", () => {
                this.value = $input.val();
            });
            $setting.find(".reset").click(() => {
                this.value = this.defaultValue;
                $input.val(this.value);
            });
        }
    }

    export class Settings {
        // noinspection JSMethodCanBeStatic
        add(setting: Setting) {
            setting.addTo($settingsContainer);
        }
    }

    export const settings = new Settings();
    let $settings: JQuery;
    let $settingsContainer: JQuery;

    internal.on_init.push(() => {
        $settings = $player.find(".settings");
        $settingsContainer = $settings.find(".settings-container");
        $player.find(".settings-button").click(() => {
            internal.toggleWidget($settings);
        });
    });
}