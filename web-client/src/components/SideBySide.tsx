import { h } from 'preact';

export function SideBySide(): h.JSX.Element {
    return (
        // TODO: use grid classes from base
        <div className="sys">
            <div className="row row-no-padding">
                <div className="column column-50">
                    <code contentEditable={true} style="min-height: 100%; padding: 8em 2em 0 2em;">
                        {`# Top-level header: my thoughts on the things`}
                    </code>
                </div>
                <div className="column column-50">
                    <div style="width: 25em; margin: 0 auto">
                        <h1>
                            Top-level header:
                            <br /> <small>my thoughts on the things</small>
                        </h1>
                    </div>
                </div>
            </div>
            <div className="row row-no-padding">
                <div className="column column-50">
                    <code contentEditable={true} style="min-height: 100%; padding: 0 2em;">
                        {`Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. `}
                    </code>
                </div>
                <div className="column column-50">
                    <div style="width: 25em; margin: 0 auto">
                        <p>
                            Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla
                            pariatur.
                        </p>
                    </div>
                </div>
            </div>
            <div className="row row-no-padding">
                <div className="column column-50">
                    <code contentEditable={true} style="min-height: 100%; padding: 0 2em;">
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut
                        labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco
                        laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in
                        voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat
                        non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
                    </code>
                </div>
                <div className="column column-50">
                    <div style="width: 25em; margin: 0 auto">
                        <p>
                            Lorem ipsum dolor sit amet, <a href="#important">consectetur</a> adipiscing elit, sed do
                            eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis
                            nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute
                            irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.
                            Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit
                            anim id est laborum. H<sub>2</sub>O
                        </p>

                        <h4>Sub-section</h4>
                        <p>
                            Lorem ipsum dolor sit amet,{' '}
                            <strong>
                                consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna
                                aliqua
                            </strong>
                            . Ut enim ad<sup>[5]</sup> minim veniam, quis nostrud exercitation ullamco laboris nisi ut
                            aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit
                            esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident,
                            sunt in culpa qui officia deserunt mollit anim id est laborum.
                        </p>

                        <h6>Sub-sub-section</h6>
                        <code>
                            {`pub async fn update_task(
        id: u64,
        task_update: NewTask,
        session_token: String,
        _db_pool: PgPool,
    ) -> Result<impl warp::Reply, Infallible> {
        debug!(
            "update_task: token={}, id={}, task_update={:?}",
            session_token, id, task_update
        );
        Ok(StatusCode::OK)
    }
    `}
                        </code>
                        <ul>
                            <li>lists!</li>
                            <li>they{'\u2019r'}e fun for organizing information</li>
                            <li>
                                Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor
                                incididunt ut labore et dolore magna aliqua.
                            </li>
                        </ul>

                        <p>
                            Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut
                            labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco
                            laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in
                            voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                            cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
                        </p>

                        <blockquote>
                            ...et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco
                            laboris.
                        </blockquote>
                        <p>
                            Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut
                            labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco
                            laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in
                            voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                            cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
                        </p>

                        <table>
                            <thead>
                                <tr>
                                    <th>Hello world</th>
                                    <th>Column 2</th>
                                    <th>Column 3</th>
                                </tr>
                            </thead>
                            <tbody>
                                <tr>
                                    <td>value 1</td>
                                    <td>value 2</td>
                                    <td>value 3</td>
                                </tr>
                                <tr>
                                    <td>value 4</td>
                                    <td>value 5</td>
                                    <td>value 6</td>
                                </tr>
                            </tbody>
                        </table>
                        <p>
                            <img src="https://placedog.net/1000" style="min-height: 10em; width: 25em" alt="dog" />
                        </p>
                        <p>
                            Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut
                            labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco
                            laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in
                            voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                            cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
                        </p>
                        <p id="important">
                            Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut
                            labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco
                            laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in
                            voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
                            cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
                        </p>

                        <hr />

                        <form>
                            <fieldset>
                                <label htmlFor="text">Text area</label>
                                <textarea id="text" />
                            </fieldset>

                            <fieldset>
                                <label htmlFor="check">Checkbox</label>
                                <input type="checkbox" id="check" />
                            </fieldset>

                            <fieldset>
                                <input type="submit" id="submit" value="Create" />
                            </fieldset>

                            <fieldset>
                                <input type="submit" disabled id="submit" value="Create (disabled)" />
                            </fieldset>

                            <fieldset>
                                <select>
                                    <option>Very Good</option>
                                    <option>Less Good</option>
                                    <option>Not Very Good</option>
                                    <option>Bad</option>
                                </select>
                            </fieldset>

                            <progress value="0.50" />
                        </form>
                    </div>
                </div>
            </div>
        </div>
    );
}
