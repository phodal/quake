/* eslint-disable */
/* tslint:disable */
/**
 * This is an autogenerated file created by the Stencil compiler.
 * It contains typing information for all components that exist in this project.
 */
import { HTMLStencilElement, JSXBase } from "@stencil/core/internal";
export namespace Components {
    interface QuakeGraph {
    }
    interface QuakeRender {
        "content": string;
    }
}
declare global {
    interface HTMLQuakeGraphElement extends Components.QuakeGraph, HTMLStencilElement {
    }
    var HTMLQuakeGraphElement: {
        prototype: HTMLQuakeGraphElement;
        new (): HTMLQuakeGraphElement;
    };
    interface HTMLQuakeRenderElement extends Components.QuakeRender, HTMLStencilElement {
    }
    var HTMLQuakeRenderElement: {
        prototype: HTMLQuakeRenderElement;
        new (): HTMLQuakeRenderElement;
    };
    interface HTMLElementTagNameMap {
        "quake-graph": HTMLQuakeGraphElement;
        "quake-render": HTMLQuakeRenderElement;
    }
}
declare namespace LocalJSX {
    interface QuakeGraph {
    }
    interface QuakeRender {
        "content"?: string;
    }
    interface IntrinsicElements {
        "quake-graph": QuakeGraph;
        "quake-render": QuakeRender;
    }
}
export { LocalJSX as JSX };
declare module "@stencil/core" {
    export namespace JSX {
        interface IntrinsicElements {
            "quake-graph": LocalJSX.QuakeGraph & JSXBase.HTMLAttributes<HTMLQuakeGraphElement>;
            "quake-render": LocalJSX.QuakeRender & JSXBase.HTMLAttributes<HTMLQuakeRenderElement>;
        }
    }
}
