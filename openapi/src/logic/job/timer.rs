use anyhow::Result;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, JoinType, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait, Set,
};

use super::{types::JobTimerRelatedJobModel, JobLogic};
use crate::entity::{job, job_exec_history, job_timer, prelude::*};

impl<'a> JobLogic<'a> {
    pub async fn save_job_timer(
        &self,
        active_model: job_timer::ActiveModel,
    ) -> Result<job_timer::ActiveModel> {
        Ok(active_model.save(&self.ctx.db).await?)
    }

    pub async fn query_job_timer(
        &self,
        created_user: Option<&String>,
        name: Option<String>,
        job_type: Option<String>,
        updated_time_range: Option<(String, String)>,
        page: u64,
        page_size: u64,
    ) -> Result<(Vec<JobTimerRelatedJobModel>, u64)> {
        let model = job_timer::Entity::find()
            .column_as(job::Column::Name, "job_name")
            .column(job::Column::ExecutorId)
            .join_rev(
                JoinType::LeftJoin,
                Job::belongs_to(JobTimer)
                    .from(job::Column::Eid)
                    .to(job_timer::Column::Eid)
                    .into(),
            )
            .apply_if(name, |query, v| {
                query.filter(job_timer::Column::Name.contains(v))
            })
            .apply_if(created_user, |query, v| {
                query.filter(job_timer::Column::CreatedUser.eq(v))
            })
            .apply_if(job_type, |query, v| {
                query.filter(job_timer::Column::JobType.eq(v))
            })
            .apply_if(updated_time_range, |query, v| {
                query.filter(
                    job_timer::Column::UpdatedTime
                        .gt(v.0)
                        .and(job_timer::Column::UpdatedTime.lt(v.1)),
                )
            });

        let total = model.clone().count(&self.ctx.db).await?;

        let list = model
            .order_by_desc(job_timer::Column::Id)
            .into_model()
            .paginate(&self.ctx.db, page_size)
            .fetch_page(page)
            .await?;

        Ok((list, total))
    }

    pub async fn delete_job_timer(&self, eid: String) -> Result<u64> {
        let record = JobExecHistory::find()
            .filter(job_exec_history::Column::Eid.eq(&eid))
            .one(&self.ctx.db)
            .await?;
        if record.is_some() {
            anyhow::bail!("forbidden to delete the executed jobs")
        }

        let ret = JobTimer::delete(job_timer::ActiveModel {
            eid: Set(eid),
            ..Default::default()
        })
        .exec(&self.ctx.db)
        .await?;
        Ok(ret.rows_affected)
    }
}
