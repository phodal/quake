/* eslint-disable */
/* tslint:disable */
/**
 * This is an autogenerated file created by the Stencil compiler.
 * It contains typing information for all components that exist in this project.
 */
import { HTMLStencilElement, JSXBase } from "@stencil/core/internal";
export namespace Components {
    interface TypeCreator {
    }
}
declare global {
    interface HTMLTypeCreatorElement extends Components.TypeCreator, HTMLStencilElement {
    }
    var HTMLTypeCreatorElement: {
        prototype: HTMLTypeCreatorElement;
        new (): HTMLTypeCreatorElement;
    };
    interface HTMLElementTagNameMap {
        "type-creator": HTMLTypeCreatorElement;
    }
}
declare namespace LocalJSX {
    interface TypeCreator {
        "onSaveProps"?: (event: CustomEvent<any>) => void;
    }
    interface IntrinsicElements {
        "type-creator": TypeCreator;
    }
}
export { LocalJSX as JSX };
declare module "@stencil/core" {
    export namespace JSX {
        interface IntrinsicElements {
            "type-creator": LocalJSX.TypeCreator & JSXBase.HTMLAttributes<HTMLTypeCreatorElement>;
        }
    }
}
