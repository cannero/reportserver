import React from 'react';
import ReactDOM from 'react-dom';
import {Nav, NavItem, Navbar} from 'react-bootstrap';
import img from './sidebar-2.jpg';
import './scss/main.scss';

class Sidebar extends React.Component {

    constructor(props){
        super(props);
    }

    render() {
        return (
            <div className="sidebar">
                <div className="sidebar-wrapper">
                    <Nav bsStyle="pills" stacked className="sidebar-nav">
                        <NavItem eventKey={1} href="#">Link 1</NavItem>
                        <NavItem eventKey={2} href="#">Link 2</NavItem>
                    </Nav>
                </div>
                <div className="sidebar-background" style={{backgroundImage: "url(" + img + ")"}}>
                 </div>
            </div>
        );
    }
}

export default Sidebar;
