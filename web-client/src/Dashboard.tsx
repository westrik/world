import * as React from 'react';

import './style/dashboard.css';

// TODO:
// - refactor Dashboard into separate components for:
//    - navigation
//    - timeline
// - add an Icon component that uses the feather-icon SVGs

interface IProps {
  siteName: string;
  onSignOut: () => void;
}

const Dashboard: React.FC<IProps> = props => (
  <div>
    <nav className="navbar navbar-dark fixed-top bg-dark flex-md-nowrap p-0 shadow">
      <a className="navbar-brand col-sm-3 col-md-2 mr-0" href="#">
        {props.siteName}
      </a>
      <input
        className="form-control form-control-dark w-100"
        type="text"
        placeholder="search"
        aria-label="search"
      />
      <ul className="navbar-nav px-3">
        <li className="nav-item text-nowrap">
          <a
            className="nav-link"
            href="#"
            onClick={() => {
              props.onSignOut();
            }}
          >
            Sign out
          </a>
        </li>
      </ul>
    </nav>

    <div className="container-fluid">
      <div className="row">
        <nav className="col-md-2 d-none d-md-block bg-light sidebar">
          <div className="sidebar-sticky">
            <ul className="nav flex-column">
              <li className="nav-item">
                <a className="nav-link active" href="#">
                  Tasks <span className="sr-only">(current)</span>
                </a>
              </li>
              <li className="nav-item">
                <a className="disabled nav-link" href="#">
                  Bookmarks
                </a>
              </li>
              <li className="nav-item">
                <a className="disabled nav-link" href="#">
                  Documents
                </a>
              </li>
            </ul>

            <h6 className="sidebar-heading d-flex justify-content-between align-items-center px-3 mt-4 mb-1 text-muted">
              <span>Admin</span>
              <a
                className="d-flex align-items-center text-muted"
                href="#"
                aria-label="Admin"
              ></a>
            </h6>
            <ul className="nav flex-column mb-2">
              <li className="nav-item">
                <a className="disabled nav-link" href="#">
                  Users
                </a>
              </li>
            </ul>
          </div>
        </nav>

        <main role="main" className="col-md-9 ml-sm-auto col-lg-10 px-4">
          <div className="d-flex justify-content-between flex-wrap flex-md-nowrap align-items-center pt-3 pb-2 mb-3 border-bottom">
            <h1 className="h2">Tasks</h1>
            <div className="btn-toolbar mb-2 mb-md-0">
              <button
                type="button"
                className="btn btn-sm btn-outline-secondary"
                data-toggle="modal"
                data-target="#createTaskModal"
              >
                Create
              </button>
            </div>
          </div>
        </main>
      </div>
    </div>

    <div
      className="modal fade"
      id="createTaskModal"
      tabIndex={-1}
      role="dialog"
      aria-labelledby="createTaskModalTitle"
      aria-hidden="true"
    >
      <div className="modal-dialog modal-dialog-centered" role="document">
        <div className="modal-content">
          <div className="modal-header">
            <h5 className="modal-title" id="createTaskModalTitle">
              Create task
            </h5>
            <button
              type="button"
              className="close"
              data-dismiss="modal"
              aria-label="Close"
            >
              <span aria-hidden="true">&times;</span>
            </button>
          </div>
          <div className="modal-body">TODO: create task form</div>
          <div className="modal-footer">
            <button
              type="button"
              className="btn btn-secondary"
              data-dismiss="modal"
            >
              Close
            </button>
            <button
              type="button"
              className="btn btn-primary"
              data-dismiss="modal"
            >
              Create
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
);

export default Dashboard;
