import React from 'react'
import PropTypes from 'prop-types'
import ReactDOM from 'react-dom'
import { KBarAnimator, KBarPortal, KBarPositioner, KBarProvider, KBarSearch } from 'kbar'
import reactToWebComponent from 'react-to-webcomponent'

const MyKbar: React.FC = ({children}) => {
    return (
        <KBarProvider>
            <KBarPortal>
                <KBarPositioner>
                    <KBarAnimator>
                        <KBarSearch />
                    </KBarAnimator>
                </KBarPositioner>
            </KBarPortal>
            {children}
        </KBarProvider>
    )
}

MyKbar.propTypes = {
    children: PropTypes.element.isRequired
}

const QuakeKbar = reactToWebComponent(MyKbar, React as any, ReactDOM as any)
customElements.define('quake-kbar', QuakeKbar as any)
