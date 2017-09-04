import React from 'react';
import ReactDOM from 'react-dom';
import {Nav, NavItem, Navbar} from 'react-bootstrap';
import './Sidebar.css';

class Sidebar extends React.Component {

    constructor(props){
        super(props);
    }

    render() {
        return (
            <Navbar inverse fluid={true} role="Navigation" className="sidebar">
                <Navbar.Header>
                    <Navbar.Text>
                        The Report
                    </Navbar.Text>
                </Navbar.Header>
                <Nav>
                    <NavItem eventKey={1} href="#">Link</NavItem>
                    <NavItem eventKey={2} href="#">Link</NavItem>
                </Nav>
            </Navbar>
        );
    }
}

export default Sidebar;
