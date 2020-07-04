import { h } from 'preact';

interface Props {
    title: string;
    submitTitle: string;
    textFieldPlaceholder?: string;
    onSubmit: () => void;
    onChange: (e: h.JSX.TargetedEvent<HTMLInputElement>) => void;
}

export default function Modal(props: Props): h.JSX.Element {
    return (
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
                            {props.title}
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
                            placeholder={props.textFieldPlaceholder}
                            required
                            onChange={props.onChange}
                        />
                    </div>
                    <div className="modal-footer">
                        <button type="button" className="btn btn-secondary" data-dismiss="modal">
                            close
                        </button>
                        <button type="button" className="btn btn-primary" onClick={props.onSubmit}>
                            {props.submitTitle}
                        </button>
                    </div>
                </div>
            </div>
        </div>
    );
}
