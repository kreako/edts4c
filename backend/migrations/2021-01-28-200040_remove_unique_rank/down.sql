ALTER TABLE domains ADD CONSTRAINT domains_rank_key UNIQUE(rank);
ALTER TABLE components ADD CONSTRAINT components_domain_id_rank_key UNIQUE(domain_id, rank);
ALTER TABLE competencies ADD CONSTRAINT competencies_component_id_rank_key UNIQUE(component_id, rank);
