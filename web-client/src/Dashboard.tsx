import { h } from 'preact';
import { useContext, useEffect, useState } from 'preact/hooks';
import './style/dashboard.css';
import Auth from './auth/AuthContext';
import { API_HOST, SITE_NAME } from './config';

// TODO:
// - refactor Dashboard into separate components for:
//    - navigation
//    - timeline
// - add an Icon component that uses the feather-icon SVGs

interface Item {
    content: string;
}

interface GetItemsResponse {
    error: string | null;
    items: Item[];
}

async function createItem(token: string, content: string): Promise<Item> {
    const response = await fetch(`${API_HOST}/item`, {
        body: JSON.stringify({ content }),
        headers: {
            Authorization: token,
            'Content-Type': 'application/json',
        },
        method: 'POST',
    });
    return await response.json();
}

function Dashboard(): h.JSX.Element {
    const [newItemContent, setNewItemContent] = useState('');
    const [items, setItems] = useState([] as Item[]);
    const authContext = useContext(Auth);

    async function getItems(token: string): Promise<void> {
        const response = await fetch(`${API_HOST}/item`, {
            // body: JSON.stringify({ }),
            // cache: 'no-cache', // *default, no-cache, reload, force-cache, only-if-cached
            // credentials: 'same-origin', // include, *same-origin, omit
            headers: {
                Authorization: token,
                'Content-Type': 'application/json',
            },
            method: 'GET', // *GET, POST, PUT, DELETE, etc.
            mode: 'cors', // no-cors, *cors, same-origin
            // redirect: 'follow', // manual, *follow, error
            // referrerPolicy: 'no-referrer', // no-referrer, *client
        });
        const resp = (await response.json()) as GetItemsResponse;
        // tslint:disable-next-line:no-console
        console.log(resp);
        if (resp.items) {
            setItems(resp.items);
        }
        // TODO: handle error
    }

    useEffect(() => {
        if (!items.length) {
            const fetch = async (): Promise<void> => {
                // tslint:disable-next-line:no-console
                console.log('running getItems');
                // TODO: redirect to /login if authToken is expired / null
                await getItems(authContext.authToken!);
            };
            fetch();
        }
    });

    return (
        <div>
            <nav className="navbar navbar-dark fixed-top bg-dark flex-md-nowrap p-0 shadow">
                <a className="navbar-brand col-sm-3 col-md-2 mr-0" href="#">
                    {SITE_NAME}
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
                            onClick={(): void => {
                                authContext.handleSignOut();
                            }}
                        >
                            Sign out
                        </a>
                    </li>
                </ul>
            </nav>

            <div className="container-fluid">
                <div className="row">
                    {/* TODO: show nav somehow in mobile viewports */}
                    <nav className="col-md-2 d-none d-md-block xs-block bg-light sidebar">
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
                                <a className="d-flex align-items-center text-muted" href="#" aria-label="Admin" />
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

                        <ul>
                            {items.map((item, key) => {
                                return <li key={key}>{item.content}</li>;
                            })}
                        </ul>
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
                            <button type="button" className="close" data-dismiss="modal" aria-label="Close">
                                <span aria-hidden="true">&times;</span>
                            </button>
                        </div>
                        <div className="modal-body">
                            <input
                                type="text"
                                id="inputContent"
                                className="form-control"
                                placeholder="Description"
                                required
                                onChange={(e): void => {
                                    setNewItemContent((e.target as HTMLInputElement).value);
                                }}
                            />
                        </div>
                        <div className="modal-footer">
                            <button type="button" className="btn btn-secondary" data-dismiss="modal">
                                Close
                            </button>
                            <button
                                type="button"
                                className="btn btn-primary"
                                onClick={(): void => {
                                    createItem(authContext.authToken!, newItemContent);
                                }}
                            >
                                Create
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
}

export default Dashboard;
