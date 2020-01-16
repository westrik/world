import { h } from 'preact';
import { useContext, useEffect, useState } from 'preact/hooks';
import Auth from '../auth/AuthContext';
import { API_HOST } from '../config';
import Container from '../components/Container';
import Modal from '../components/Modal';

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

function TasksListing(): h.JSX.Element {
    const [newItemContent, setNewItemContent] = useState('');
    const [items, setItems] = useState([] as Item[]);
    const authContext = useContext(Auth);

    async function getItems(token: string): Promise<void> {
        const response = await fetch(`${API_HOST}/item`, {
            headers: {
                Authorization: token,
                'Content-Type': 'application/json',
            },
            method: 'GET',
        });
        const resp = (await response.json()) as GetItemsResponse;
        if (resp.items) {
            setItems(resp.items);
        }
        // TODO: handle error
    }

    useEffect(() => {
        if (!items.length) {
            const fetch = async (): Promise<void> => {
                // TODO: redirect to /login if authToken is expired / null
                await getItems(authContext.authToken!);
            };
            fetch();
        }
    });

    return (
        <Container>
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

            <Modal
                onChange={(e): void => {
                    setNewItemContent((e.target as HTMLInputElement).value);
                }}
                onSubmit={(): void => {
                    createItem(authContext.authToken!, newItemContent);
                }}
            />
        </Container>
    );
}

export default TasksListing;
