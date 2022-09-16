import {BaseHTMLElement, customElement, getFirst, html} from 'dom-native';

@customElement("task-view")
class TaskView extends BaseHTMLElement {
    #taskInputElement!:TaskInput;
    #taskListElement!:HTMLElement;

    init(){
        let htmlContent: DocumentFragment = html`
            <div><div>
            <h1>Görevler</h1>
            <task-input></task-input>
            <task-list>/<task-list>
        `;
        [this.#taskInputElement,this.#taskListElement] = getFirst(htmlContent,'task-input','task-list');
        this.append(htmlContent);
        this.refresh();
    }

    async refresh() {
        let task_list=[
            {id:1,title:"Bulaşıkları yıka",state:"Ready"},
            {id:2,title:"10 Km koş",state:"Inprogress"}
        ];

        let htmlContent=document.createDocumentFragment();
        for(const task of task_list){
            const ti=document.createElement('task-item');
            htmlContent.append(ti);
        }

        this.#taskListElement.innerHTML = '';
        this.#taskListElement.append(htmlContent);
    }
}

@customElement("task-input")
class TaskInput extends BaseHTMLElement {
    #inputEl!: HTMLInputElement;

    init(){
        let htmlContent = html`
            <input type="text" placeholder="Sıkıcı bir görev ekleyebilirsin?">
        `;
        this.#inputEl = getFirst(htmlContent,'input');
        this.append(htmlContent);
    }
}

declare global{
    interface HTMLElementTagNameMap{
        'task-input': TaskInput;
    }
}

@customElement("task-item")
class TaskItem extends BaseHTMLElement {
    #titleEl!: HTMLElement;

    init(){
        let htmlContent = html`
            <c-check><c-ico name="ico-done"></c-ico></c-check>
            <div>Görev Başlığı Gelecek</div>
            <c-ico name="del"></c-ico>
        `;
        this.#titleEl = getFirst(htmlContent,'div');
        this.append(htmlContent);
    }
}

declare global{
    interface HTMLElementTagNameMap{
        'task-item': TaskItem;
    }
}

