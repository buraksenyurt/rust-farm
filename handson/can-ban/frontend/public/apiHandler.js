import {WorkItemManager} from './pkg/can_ban.js';

export async function createWorkItem(workItemData) {
    const manager = WorkItemManager.new();
    console.log(workItemData);
    const payload = await manager
        .create(
            workItemData.title
            , workItemData.duration
            , workItemData.durationType
            , workItemData.size);
    return JSON.parse(payload);
}