import {
  Button,
  Container,
  Grid,
  NumberInput,
  Select,
  TextInput,
} from '@mantine/core';
import { FC } from 'react';
import { SubmitHandler, useFieldArray, useForm } from 'react-hook-form';
import { saveCarryInfoData } from '../ext';

interface CarryDistributionFormInputs {
  code: string;
  monkeys: Array<{ id: string }>;
}

const CarryDistributionForm: FC = () => {
  const {
    control,
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<CarryDistributionFormInputs>({});

  const { fields } = useFieldArray({
    control,
    name: 'monkeys',
    keyName: 'id',
  });

  const onSubmit: SubmitHandler<CarryDistributionFormInputs> = async (data) => {
    console.log(data);

    const x = await saveCarryInfoData();

    console.log({ x });
  };

  return (
    <Container>
      <form onSubmit={handleSubmit(onSubmit)}>
        <Select
          placeholder="hearsay"
          label="Source"
          searchable
          clearable
          required
          data={[{ value: '1', label: '1' }]}
        />
        <Grid>
          <Grid.Col span={2}>
            <NumberInput
              hideControls
              defaultValue={10}
              placeholder="$"
              label="$"
            />
          </Grid.Col>
          <Grid.Col span={10}>
            <TextInput
              placeholder="0x0x0x0x0"
              label="Carry Code"
              required
              {...register('code', { minLength: 6 })}
            />
          </Grid.Col>
        </Grid>
        {Array(4)
          .fill(0)
          .map((_, index) => (
            <Grid>
              <Grid.Col span={2}>
                <NumberInput
                  defaultValue={1}
                  placeholder="Parts"
                  label="Parts"
                  hideControls
                />
              </Grid.Col>
              <Grid.Col span={2}>
                <NumberInput
                  defaultValue={1}
                  placeholder="Amount"
                  label="Amount"
                  disabled
                />
              </Grid.Col>
              <Grid.Col span={8}>
                <Select
                  placeholder="Empty"
                  label={`Monkey ${index + 1}`}
                  searchable
                  clearable
                  data={[{ value: '1', label: '1' }]}
                />
              </Grid.Col>
            </Grid>
          ))}
        <Button variant="outline" type="submit">
          Create!
        </Button>
      </form>
      {errors.code && <p>error</p>}
    </Container>
  );
};

export default CarryDistributionForm;
